# 🔌 Calclaw עם Nebius AI Factory וספקי API נוספים

## 🇮🇱 Nebius AI Factory - הספק הישראלי עם המון API Options

### 📖 מבוא
**Nebius AI Factory** הוא ספק API ישראלי מצוין שמציע **המון API options** במחירים תחרותיים, עם תמיכה בעברית מלאה ושעות פעילות לפי שעון ישראל.

### 🎯 למה Nebius AI Factory?

#### ✅ **יתרונות לקהל הישראלי:**
- **🇮🇱 ספק ישראלי** - תמיכה בעברית מלאה
- **🕐 שעות פעילות** - א'-ה' 9:00-18:00 לפי שעון ישראל
- **📞 תמיכה טלפונית** - בשפה העברית
- **💼 התאמה לשוק הישראלי** - מחירים בשקלים, תשלום בכרטיס ישראלי
- **🔒 אבטחה ישראלית** - לפי תקנים מקומיים

#### ✅ **יתרונות טכניים:**
- **🔌 Unified API gateway** - גישה אחת לכל ה-APIs
- **⚡ Latency נמוך** - שרתים באירופה/ישראל
- **📊 ניטור מתקדם** - דשבורדים בעברית
- **💸 מחירים תחרותיים** - זולים מ-OpenAI בחלק מהשירותים

### 🎯 API Options של Nebius

#### 1. 🤖 **Text Generation APIs**
- **GPT-4 API** - הגרסה המתקדמת ביותר
- **Claude API** - מתמחה בבטיחות וארוך-הקשר
- **Gemini API** - אינטגרציה עם Google
- **Llama API** - מודלים פתוחים
- **Mistral API** - מודלים אירופאים

#### 2. 🎨 **Image Generation APIs**
- **DALL-E 3** - יצירת תמונות באיכות גבוהה
- **Stable Diffusion** - יצירת תמונות עם בקרה מתקדמת
- **Midjourney API** - דרך Nebius (בתשלום נוסף)

#### 3. 🎤 **Speech Processing APIs**
- **Speech-to-Text** - המרת קול לטקסט
- **Text-to-Speech** - המרת טקסט לקול
- **Voice Cloning** - שיבוט קול (בתשלום נוסף)

#### 4. 👁️ **Computer Vision APIs**
- **Object Detection** - זיהוי עצמים בתמונות
- **Image Classification** - סיווג תמונות
- **Face Recognition** - זיהוי פנים
- **OCR (Optical Character Recognition)** - זיהוי טקסט בתמונות

#### 5. 📈 **Data Analysis APIs**
- **Data Visualization** - יצירת גרפים ודשבורדים
- **Statistical Analysis** - ניתוח סטטיסטי
- **Predictive Modeling** - מודלים חיזוי

### 💰 מחירים ותמחור

#### 🆓 **Tier חינמי:**
- 1,000 requests לחודש
- 10 דקות Speech-to-Text
- 5 תמונות DALL-E
- מושלם לניסוי והתחלה

#### 💸 **Tier בסיסי ($29/חודש):**
- 10,000 requests לחודש
- 100 דקות Speech-to-Text
- 50 תמונות DALL-E
- תמיכה באימייל

#### 🏢 **Tier עסקי ($99/חודש):**
- 100,000 requests לחודש
- 1,000 דקות Speech-to-Text
- 500 תמונות DALL-E
- תמיכה טלפונית
- SLA 99.9%

#### 🏛️ **Tier ארגוני ($299/חודש):**
- 1,000,000 requests לחודש
- 10,000 דקות Speech-to-Text
- 5,000 תמונות DALL-E
- תמיכה ייעודית
- SLA 99.99%
- Compliance מלא

### 🚀 איך מתחילים עם Nebius?

#### שלב 1: רישום
```bash
# קישור ישיר לרישום:
https://console.nebius.ai/signup

# או הרץ את ההדרכה:
./scripts/setup_nebius_api.sh
```

#### שלב 2: יצירת API Key
1. היכנס ל-Console: https://console.nebius.ai
2. עבור ל: **API Keys** → **Create New Key**
3. בחר את ה-API types שאתה צריך
4. שמור את ה-API key במקום בטוח

#### שלב 3: הגדרה ב-Calclaw
```bash
# ערוך את קובץ ה-credentials:
nano config/api_providers/nebius_credentials.json

# הוסף את ה-API key שלך:
{
  "api_key": "YOUR_NEW_API_KEY_HERE",
  "organization_id": "",
  "project_id": "",
  "environment": "production"
}
```

#### שלב 4: בדיקת חיבור
```bash
# בדוק את החיבור:
./scripts/test_nebius_api.sh

# או בדוק את כל ה-APIs:
./scripts/test_all_apis.sh
```

### 🎯 דוגמאות לשימוש

#### שיחה עם GPT-4 דרך Nebius:
```bash
curl -X POST http://localhost:3003/api/nebius/chat \
  -H "Content-Type: application/json" \
  -d '{
    "provider": "gpt-4",
    "message": "תסביר לי על Nebius AI Factory בעברית",
    "language": "hebrew",
    "temperature": 0.7
  }'
```

#### יצירת תמונה עם DALL-E:
```bash
curl -X POST http://localhost:3003/api/nebius/images/generate \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "נוף ישראלי עם הרים, שדות, ושמיים כחולים",
    "model": "dall-e-3",
    "size": "1024x1024",
    "quality": "standard",
    "style": "vivid"
  }'
```

#### המרת קול לטקסט:
```bash
curl -X POST http://localhost:3003/api/nebius/speech/transcribe \
  -H "Content-Type: multipart/form-data" \
  -F "file=@audio_file.mp3" \
  -F "language=he"
```

#### ניתוח תמונה:
```bash
curl -X POST http://localhost:3003/api/nebius/vision/analyze \
  -H "Content-Type: multipart/form-data" \
  -F "file=@image.jpg" \
  -F "tasks=object_detection,face_recognition,ocr"
```

## 🤖 ספקי API נוספים

### OpenAI
- **GPT-4 Turbo** - המודל המתקדם ביותר
- **GPT-4 Vision** - עם יכולות ראייה
- **DALL-E 3** - יצירת תמונות
- **Whisper** - Speech-to-Text
- **🔗 רישום:** https://platform.openai.com/signup

### Anthropic (Claude)
- **Claude 3 Opus** - המודל המתקדם ביותר
- **Claude 3 Sonnet** - מאוזן בין מחיר לביצועים
- **Claude 3 Haiku** - מהיר וזול
- **מתמחה בבטיחות** - Constitutional AI
- **🔗 רישום:** https://console.anthropic.com/signup

### Google Gemini
- **Gemini Pro** - מודל טקסט מתקדם
- **Gemini Vision** - עם יכולות ראייה
- **אינטגרציה** עם Google Cloud
- **🔗 רישום:** https://makersuite.google.com/app/apikey

### Microsoft Azure AI
- **Azure OpenAI Service** - GPT-4 בענן של Microsoft
- **Azure Cognitive Services** - שירותי AI מלאים
- **Compliance מלא** - GDPR, HIPAA, SOC2
- **לארגונים גדולים** - עם SLA גבוה
- **🔗 רישום:** https://azure.microsoft.com/services/openai

## 🔌 API Gateway של Calclaw

### 🎯 מה זה API Gateway?
**API Gateway** של Calclaw הוא שכבת ביניים חכמה שמנהלת את כל ספקי ה-API:

- **🔀 Load Balancing** - חלוקת עומסים בין providers
- **💸 Cost Optimization** - בחירת הספק הזול ביותר לכל בקשה
- **⚡ Fallback Handling** - מעבר אוטומטי לספק גיבוי
- **📊 Analytics** - ניתוח שימוש ועלויות
- **🔒 Security** - ניהול API keys בצורה מאובטחת

### 🚀 איך עובד ה-API Gateway?

#### 1. **בחירה חכמה של ספק:**
```rust
// דוגמת קוד פשוטה:
fn select_provider(task: &Task) -> Provider {
    match task.requirements {
        Requirement::LowCost => Provider::Nebius,      // Nebius זול יותר
        Requirement::HighQuality => Provider::OpenAI,  // OpenAI איכותי יותר
        Requirement::HebrewSupport => Provider::Nebius, // Nebius תומך בעברית
        Requirement::Enterprise => Provider::Azure,    // Azure לארגונים
    }
}
```

#### 2. **Fallback אוטומטי:**
```rust
// אם ספק אחד נכשל, עובר לספק אחר:
async fn execute_with_fallback(task: Task) -> Result<Response> {
    let providers = vec![Provider::Nebius, Provider::OpenAI, Provider::Anthropic];
    
    for provider in providers {
        match provider.execute(&task).await {
            Ok(response) => return Ok(response),
            Err(_) => continue, // נסה את הספק הבא
        }
    }
    
    Err("All providers failed")
}
```

#### 3. **Cost optimization:**
```rust
// בחירת הספק הזול ביותר:
fn optimize_cost(task: &Task) -> Provider {
    let costs = HashMap::from([
        (Provider::Nebius, calculate_nebius_cost(task)),
        (Provider::OpenAI, calculate_openai_cost(task)),
        (Provider::Anthropic, calculate_anthropic_cost(task)),
    ]);
    
    costs.iter()
        .min_by_key(|(_, &cost)| cost)
        .map(|(&provider, _)| provider)
        .unwrap_or(Provider::Nebius)
}
```

### 📊 דשבורד API Gateway

#### סטטוס providers:
```
🔌 API Gateway Status
====================

🇮🇱 Nebius AI Factory:
  • Status: ✅ Online
  • Latency: 45ms
  • Cost: $0.002/request
  • Usage: 1,234/10,000 requests

🤖 OpenAI:
  • Status: ✅ Online  
  • Latency: 120ms
  • Cost: $0.003/request
  • Usage: 567/5,000 requests

🎯 Anthropic:
  • Status: ⚠️  High Latency (280ms)
  • Latency: 280ms
  • Cost: $0.004/request
  • Usage: 89/1,000 requests

📊 Summary:
  • Total Requests: 1,890
  • Total Cost: $5.67
  • Avg Latency: 98ms
  • Success Rate: 99.2%
```

## 🎯 השוואה בין ספקים

| ספק | מחיר (per 1K tokens) | Latency | תמיכה בעברית | יתרונות | חסרונות |
|------|----------------------|---------|---------------|----------|----------|
| **🇮🇱 Nebius** | $0.002 | 45ms | ✅ מלאה | מחיר זול, תמיכה ישראלית | פחות מוכר גלובלית |
| **🤖 OpenAI** | $0.003 | 120ms | ⚠️ חלקית | הכי מתקדם, הכי פופולרי | יקר יותר |
| **🎯 Anthropic** | $0.004 | 180ms | ⚠️ חלקית | בטיחות גבוהה, ארוך-הקשר | הכי יקר |
| **🔍 Google** | $0.0025 | 150ms | ✅ טובה | אינטגרציה עם Google Cloud | פחות מתקדם |
| **☁️ Azure** | $0.0035 | 100ms | ⚠️ חלקית | Compliance מלא, לארגונים | מורכב יותר |

## 🔧 ניהול API Providers ב-Calclaw

### הצגת סטטוס:
```bash
./scripts/manage_api_providers.sh status
```

### הפעלת provider:
```bash
./scripts/manage_api_providers.sh enable nebius
```

### ביטול provider:
```bash
./scripts/manage_api_providers.sh disable openai
```

### הדרכה להגדרה:
```bash
./scripts/manage_api_providers.sh setup
```

### בדיקת חיבור:
```bash
./scripts/test_all_apis.sh
```

## 📈 Best Practices

### 1. **התחלה עם Nebius:**
- התחל עם ה-tier החינמי
- נסה את כל ה-API types
- הגדר spending limits
- השתמש בתמיכה העברית אם צריך

### 2. **Optimization עלויות:**
- השתמש ב-Nebius לטקסט בעברית
- השתמש ב-OpenAI לטקסט באנגלית מתקדם
- השתמש ב-Anthropic לבטיחות גבוהה
- השתמש ב-Azure לארגונים גדולים

### 3. **ניהול API keys:**
- אל תשמור API keys בקוד
- השתמש בקובץ credentials נפרד
- סובב API keys באופן קבוע
- השתמש ב-secrets management

### 4. **Monitoring ו-alerts:**
- עקוב אחרי שימוש ועלויות
- הגדר alerts ל-spending limits
- ניטור latency ו-success rate
- דוחות חודשיים לשימוש

## 🚀 התקנה מהירה

### התקנה עם Nebius AI Factory:
```bash
# הורד את Calclaw
git clone https://github.com/nadavsimba24/calclaw.git
cd calclaw

# הרץ את ה-Smart Installer עם API providers
./install_calclaw_smart_with_providers.sh

# ענה על השאלות:
# 1️⃣ מטרה: מה שאתה צריך
# 2️⃣ יכולות: מה שאתה רוצה  
# 3️⃣ פריסה: איפה להריץ
# 4️⃣ אבטחה: איזה רמה
# 5️⃣ Ollama: האם צריך
# 6️⃣ API providers: בחר 2 (Nebius)
# 7️⃣ API keys: 1 (צריך ליצור)

# המערכת תתקין ותנחה אותך בהגדרת Nebius
```

### הגדרת Nebius:
```bash
# הרץ את ההדרכה:
./scripts/setup_nebius_api.sh

# עקוב אחרי השלבים:
# 1. רישום ל-Nebius
# 2. יצירת API key
# 3. הגדרה ב-Calclaw
# 4. בדיקת חיבור

# בדוק שהכל עובד:
./scripts/test_nebius_api.sh
```

### הרצת המערכת:
```bash
# הפעל את כל השירותים:
$HOME/.calclaw/start_calclaw.sh

# בדוק סטטוס:
$HOME/.calclaw/status_calclaw.sh

# התחל להשתמש!
```

## 📞 תמיכה

### Nebius AI Factory:
- **אתר:** https://nebius.com/ai-factory
- **דוקומנטציה:** https://docs.nebius.ai
- **תמיכה:** support@nebius.ai
- **טלפון:** *ישראלי*
- **שעות פעילות:** א'-ה' 9:00-18:00

### Calclaw:
- **GitHub:** https://github.com/nadavsimba24/calclaw
- **דוקומנטציה:** https://calclaw.ai/docs
- **Community:** https://discord.gg/calclaw
- **Issues:** https://github.com/nadavsimba24/calclaw/issues

## 🎉 סיכום

**Calclaw עם Nebius AI Factory** הוא השילוב המושלם עבור קהל ישראלי:

- **🇮🇱 ספק ישראלי** - עם תמיכה בעברית מלאה
- **🔌 המון API options** - מטקסט ועד ראייה ממוחשבת
- **💸 מחירים תחרותיים** - זולים מהתחרות
- **⚡ ביצועים מעולים** - latency נמוך
- **🎯 קל לשימוש** - עם Smart Installer

**פלוס כל ספקי ה-API המובילים:** OpenAI, Anthropic, Google, Microsoft