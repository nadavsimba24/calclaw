#!/usr/bin/env python3
"""
Ollama Proxy for CalcLaw
This script adds Ollama integration to existing CalcLaw server
"""

import asyncio
import aiohttp
from aiohttp import web
import json
import logging
from typing import Dict, Any, Optional

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class OllamaProxy:
    def __init__(self, ollama_url: str = "http://localhost:11434"):
        self.ollama_url = ollama_url
        self.session: Optional[aiohttp.ClientSession] = None
        
    async def start(self):
        """Start the HTTP session"""
        self.session = aiohttp.ClientSession()
        
    async def stop(self):
        """Stop the HTTP session"""
        if self.session:
            await self.session.close()
            
    async def health_check(self) -> Dict[str, Any]:
        """Check if Ollama is running"""
        try:
            async with self.session.get(f"{self.ollama_url}/api/tags") as response:
                if response.status == 200:
                    data = await response.json()
                    return {
                        "ollama_running": True,
                        "models_count": len(data.get("models", [])),
                        "default_models": ["phi3:mini", "gemma2:9b", "phi3:3.8b"]
                    }
                else:
                    return {"ollama_running": False, "error": f"HTTP {response.status}"}
        except Exception as e:
            logger.error(f"Health check failed: {e}")
            return {"ollama_running": False, "error": str(e)}
            
    async def list_models(self) -> Dict[str, Any]:
        """List available Ollama models"""
        try:
            async with self.session.get(f"{self.ollama_url}/api/tags") as response:
                if response.status == 200:
                    data = await response.json()
                    return {"success": True, "models": data.get("models", [])}
                else:
                    error_text = await response.text()
                    return {"success": False, "error": f"HTTP {response.status}: {error_text}"}
        except Exception as e:
            logger.error(f"List models failed: {e}")
            return {"success": False, "error": str(e)}
            
    async def generate_text(self, model: str, prompt: str, task_type: str = "general", language: str = "Python") -> Dict[str, Any]:
        """Generate text using Ollama"""
        try:
            # Prepare request based on task type
            if task_type == "hebrew":
                hebrew_prompt = f"{prompt} תשובה בעברית בבקשה."
                request_prompt = hebrew_prompt
            elif task_type == "code":
                request_prompt = f"Write {language} code for: {prompt}"
            else:
                request_prompt = prompt
                
            request_data = {
                "model": model,
                "prompt": request_prompt,
                "stream": False,
                "options": {
                    "temperature": 0.7,
                    "top_p": 0.9,
                    "top_k": 40,
                    "num_predict": 512
                }
            }
            
            logger.info(f"Generating with model {model}, prompt length: {len(request_prompt)}")
            
            async with self.session.post(f"{self.ollama_url}/api/generate", json=request_data) as response:
                if response.status == 200:
                    data = await response.json()
                    
                    # Process response for Hebrew RTL
                    response_text = data.get("response", "")
                    if task_type == "hebrew" and any('\u0590' <= c <= '\u05FF' for c in response_text):
                        response_text = f"\u202B{response_text}\u202C"
                        
                    return {
                        "success": True,
                        "model": model,
                        "response": response_text,
                        "processing_time_ms": data.get("total_duration", 0) // 1000000  # Convert nanoseconds to milliseconds
                    }
                else:
                    error_text = await response.text()
                    logger.error(f"Generation failed: HTTP {response.status}: {error_text}")
                    return {
                        "success": False,
                        "model": model,
                        "response": "",
                        "error": f"HTTP {response.status}: {error_text}"
                    }
        except Exception as e:
            logger.error(f"Generation failed: {e}")
            return {"success": False, "error": str(e)}

# HTTP handlers
async def handle_health(request):
    """Handle health check request"""
    proxy = request.app['proxy']
    result = await proxy.health_check()
    return web.json_response(result)

async def handle_models(request):
    """Handle list models request"""
    proxy = request.app['proxy']
    result = await proxy.list_models()
    return web.json_response(result)

async def handle_generate(request):
    """Handle generate text request"""
    try:
        data = await request.json()
        model = data.get('model', 'phi3:mini')
        prompt = data.get('prompt', '')
        task_type = data.get('task_type', 'general')
        language = data.get('language', 'Python')
        
        proxy = request.app['proxy']
        result = await proxy.generate_text(model, prompt, task_type, language)
        
        return web.json_response(result)
    except json.JSONDecodeError as e:
        return web.json_response({"success": False, "error": f"Invalid JSON: {e}"}, status=400)
    except Exception as e:
        return web.json_response({"success": False, "error": str(e)}, status=500)

async def handle_dashboard(request):
    """Serve Ollama dashboard HTML"""
    html = """
    <!DOCTYPE html>
    <html dir="rtl" lang="he">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>CalcLaw - Ollama Proxy</title>
        <style>
            body { font-family: Arial, sans-serif; margin: 40px; }
            .hebrew { text-align: right; direction: rtl; }
            .header { background: #2196F3; color: white; padding: 20px; border-radius: 5px; }
            .status { background: #f0f0f0; padding: 15px; margin: 10px 0; border-radius: 5px; }
            .feature { background: #e3f2fd; padding: 15px; margin: 10px 0; border-radius: 5px; }
            .form-group { margin: 10px 0; }
            label { display: block; margin-bottom: 5px; font-weight: bold; }
            input, select, textarea { width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px; }
            button { background: #4CAF50; color: white; border: none; padding: 10px 20px; border-radius: 4px; cursor: pointer; }
            button:hover { background: #45a049; }
            .response { background: #f9f9f9; padding: 15px; border-left: 4px solid #4CAF50; margin: 10px 0; }
            .error { border-left-color: #f44336; }
        </style>
        <script>
            async function checkHealth() {
                const response = await fetch('/api/ollama/health');
                const data = await response.json();
                
                const statusDiv = document.getElementById('ollama-status');
                if (data.ollama_running) {
                    statusDiv.innerHTML = `
                        <h3>✅ Ollama פועל</h3>
                        <p>📊 מספר מודלים: ${data.models_count}</p>
                        <p>🤖 מודלים מומלצים: ${data.default_models.join(', ')}</p>
                    `;
                    statusDiv.style.background = '#e8f5e9';
                } else {
                    statusDiv.innerHTML = `
                        <h3>❌ Ollama לא פועל</h3>
                        <p>אנא ודא ש-Ollama רץ על localhost:11434</p>
                        <p>הפעל עם: <code>ollama serve</code></p>
                        <p>שגיאה: ${data.error || 'לא ידוע'}</p>
                    `;
                    statusDiv.style.background = '#ffebee';
                }
            }

            async function listModels() {
                const response = await fetch('/api/ollama/models');
                const data = await response.json();
                
                const modelsDiv = document.getElementById('models-list');
                if (data.success) {
                    let html = '<h3>📚 מודלים זמינים:</h3><ul>';
                    data.models.forEach(model => {
                        const sizeGB = (model.size / 1024 / 1024 / 1024).toFixed(1);
                        html += `<li><strong>${model.name}</strong> - ${sizeGB} GB</li>`;
                    });
                    html += '</ul>';
                    modelsDiv.innerHTML = html;
                } else {
                    modelsDiv.innerHTML = `<p class="error">❌ שגיאה: ${data.error}</p>`;
                }
            }

            async function generateText() {
                const model = document.getElementById('model').value;
                const prompt = document.getElementById('prompt').value;
                const taskType = document.getElementById('task-type').value;
                const language = document.getElementById('language').value;
                
                const responseDiv = document.getElementById('generation-response');
                responseDiv.innerHTML = '<p>⚡ מעבד...</p>';
                
                const requestBody = {
                    model: model,
                    prompt: prompt,
                    task_type: taskType,
                    language: taskType === 'code' ? language : null
                };
                
                try {
                    const response = await fetch('/api/ollama/generate', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify(requestBody)
                    });
                    
                    const data = await response.json();
                    
                    if (data.success) {
                        responseDiv.innerHTML = `
                            <h3>✅ תשובה:</h3>
                            <p><strong>מודל:</strong> ${data.model}</p>
                            <p><strong>זמן עיבוד:</strong> ${data.processing_time_ms || 'לא ידוע'} ms</p>
                            <div style="background: white; padding: 15px; border-radius: 5px; margin-top: 10px;">
                                ${data.response.replace(/\\n/g, '<br>')}
                            </div>
                        `;
                        responseDiv.className = 'response';
                    } else {
                        responseDiv.innerHTML = `<p class="error">❌ שגיאה: ${data.error}</p>`;
                        responseDiv.className = 'response error';
                    }
                } catch (error) {
                    responseDiv.innerHTML = `<p class="error">❌ שגיאת רשת: ${error}</p>`;
                    responseDiv.className = 'response error';
                }
            }

            // Initialize on page load
            document.addEventListener('DOMContentLoaded', () => {
                checkHealth();
                listModels();
            });
            
            function toggleLanguageField() {
                const taskType = document.getElementById('task-type').value;
                const languageField = document.getElementById('language-field');
                languageField.style.display = taskType === 'code' ? 'block' : 'none';
            }
        </script>
    </head>
    <body>
        <div class="header">
            <h1>🤖 CalcLaw - Ollama Proxy</h1>
            <p>שילוב מודלי AI מקומיים עם CalcLaw (גרסת Python)</p>
        </div>
        
        <div id="ollama-status" class="status">
            <p>בודק סטטוס...</p>
        </div>
        
        <div class="feature">
            <h3>🤖 יצירת טקסט עם AI מקומי</h3>
            <div class="form-group">
                <label for="model">מודל:</label>
                <select id="model">
                    <option value="phi3:mini">Phi3 Mini (2.2GB) - קל ומהיר</option>
                    <option value="gemma2:9b">Gemma2:9b (5.4GB) - חזק</option>
                    <option value="phi3:3.8b">Phi3:3.8b (2.2GB) - כללי</option>
                </select>
            </div>
            
            <div class="form-group">
                <label for="task-type">סוג משימה:</label>
                <select id="task-type" onchange="toggleLanguageField()">
                    <option value="general">כללי</option>
                    <option value="hebrew">עברית</option>
                    <option value="code">קוד</option>
                </select>
            </div>
            
            <div class="form-group" id="language-field" style="display: none;">
                <label for="language">שפת תכנות:</label>
                <input type="text" id="language" value="Python" placeholder="Python, JavaScript, Rust, etc.">
            </div>
            
            <div class="form-group">
                <label for="prompt">פקודה:</label>
                <textarea id="prompt" rows="4" placeholder="מה תרצה ליצור?"></textarea>
            </div>
            
            <button onclick="generateText()">🚀 צור טקסט</button>
        </div>
        
        <div id="generation-response" class="response">
            <!-- Response will appear here -->
        </div>
        
        <div id="models-list" class="feature">
            <h3>📚 טוען רשימת מודלים...</h3>
        </div>
        
        <div class="feature">
            <h3>🔗 API Endpoints</h3>
            <div class="hebrew">
                <p><strong>GET</strong> <code>/api/ollama/health</code> - בדיקת סטטוס Ollama</p>
                <p><strong>GET</strong> <code>/api/ollama/models</code> - רשימת מודלים זמינים</p>
                <p><strong>POST</strong> <code>/api/ollama/generate</code> - יצירת טקסט עם מודל</p>
            </div>
        </div>
    </body>
    </html>
    """
    return web.Response(text=html, content_type='text/html')

async def start_proxy():
    """Start the Ollama proxy server"""
    proxy = OllamaProxy()
    await proxy.start()
    
    app = web.Application()
    app['proxy'] = proxy
    
    # Add routes
    app.router.add_get('/api/ollama/health', handle_health)
    app.router.add_get('/api/ollama/models', handle_models)
    app.router.add_post('/api/ollama/generate', handle_generate)
    app.router.add_get('/ollama', handle_dashboard)
    app.router.add_get('/', handle_dashboard)
    
    # Cleanup on shutdown
    async def on_shutdown(app):
        await proxy.stop()
    
    app.on_shutdown.append(on_shutdown)
    
    runner = web.AppRunner(app)
    await runner.setup()
    site = web.TCPSite(runner, '127.0.0.1', 3001)
    
    logger.info("🚀 Starting Ollama proxy on http://127.0.0.1:3001")
    logger.info("📊 Dashboard: http://127.0.0.1:3001/ollama")
    logger.info("🤖 API endpoints:")
    logger.info("  GET  /api/ollama/health")
    logger.info("  GET  /api/ollama/models")
    logger.info("  POST /api/ollama/generate")
    
    await site.start()
    
    # Keep running
    try:
        await asyncio.Future()  # Run forever
    except asyncio.CancelledError:
        pass
    finally:
        await runner.cleanup()

def main():
    """Main entry point"""
    try:
        asyncio.run(start_proxy())
    except KeyboardInterrupt:
        logger.info("Shutting down Ollama proxy...")
    except Exception as e:
        logger.error(f"Failed to start proxy: {e}")

if __name__ == '__main__':
    main()