#!/bin/bash

# 🐙 GitHub Setup Script for Calclaw
# עוזר להעלות את Calclaw ל-GitHub

set -e

echo "🐙 GitHub Setup for Calclaw"
echo "==========================="
echo ""

# בדוק אם git מותקן
if ! command -v git &> /dev/null; then
    echo "❌ Git לא מותקן"
    echo "📥 התקן עם: sudo apt install git"
    exit 1
fi

echo "✅ Git מותקן: $(git --version)"
echo ""

# שאל פרטים
echo "📝 פרטי GitHub:"
read -p "GitHub username: " GITHUB_USERNAME
read -p "Repository name (default: calclaw): " REPO_NAME
REPO_NAME="${REPO_NAME:-calclaw}"
read -p "Description (default: AI Assistant Framework): " REPO_DESC
REPO_DESC="${REPO_DESC:-AI Assistant Framework}"
read -p "Public repository? (y/n, default: y): " IS_PUBLIC
IS_PUBLIC="${IS_PUBLIC:-y}"

echo ""
echo "⚙️  הגדרות:"
echo "   👤 Username: $GITHUB_USERNAME"
echo "   📦 Repository: $REPO_NAME"
echo "   📝 Description: $REPO_DESC"
echo "   🌐 Public: $IS_PUBLIC"
echo ""

# שאל אם יש token
echo "🔐 GitHub Token:"
echo "   • צור token ב: https://github.com/settings/tokens"
echo "   • הרשאות נדרשות: repo, write:packages, delete_repo"
echo ""
read -p "יש לך GitHub Personal Access Token? (y/n): " HAS_TOKEN

if [[ "$HAS_TOKEN" == "y" || "$HAS_TOKEN" == "Y" ]]; then
    read -sp "הכנס את ה-token (לא יוצג): " GITHUB_TOKEN
    echo ""
    
    # שמור את ה-token זמנית
    echo "$GITHUB_TOKEN" > /tmp/github_token_calclaw.txt
    chmod 600 /tmp/github_token_calclaw.txt
else
    echo "⚠️  ללא token, ניתן רק ליצור repository מקומי"
    echo "📖 צור token ואז הרץ שוב את הסקריפט"
    exit 0
fi

# צור את ה-repository ב-GitHub
echo ""
echo "🚀 יוצר repository ב-GitHub..."

if [[ "$IS_PUBLIC" == "y" || "$IS_PUBLIC" == "Y" ]]; then
    PRIVATE="false"
else
    PRIVATE="true"
fi

# השתמש ב-GitHub API ליצירת repository
API_RESPONSE=$(curl -s -X POST \
  -H "Authorization: token $GITHUB_TOKEN" \
  -H "Accept: application/vnd.github.v3+json" \
  https://api.github.com/user/repos \
  -d "{
    \"name\": \"$REPO_NAME\",
    \"description\": \"$REPO_DESC\",
    \"private\": $PRIVATE,
    \"has_issues\": true,
    \"has_projects\": true,
    \"has_wiki\": true,
    \"auto_init\": false
  }")

# בדוק אם הצליח
if echo "$API_RESPONSE" | grep -q '"html_url"'; then
    REPO_URL=$(echo "$API_RESPONSE" | python3 -c "import sys,json;d=json.load(sys.stdin);print(d['html_url'])")
    SSH_URL=$(echo "$API_RESPONSE" | python3 -c "import sys,json;d=json.load(sys.stdin);print(d['ssh_url'])")
    
    echo "✅ Repository נוצר: $REPO_URL"
    echo ""
    
    # הוסף remote
    echo "🔗 מוסיף remote..."
    git remote add origin "$SSH_URL"
    
    # דחוף את הקוד
    echo "📤 דוחף קוד ל-GitHub..."
    git push -u origin master
    
    echo ""
    echo "🎉 הקוד הועלה ל-GitHub!"
    echo ""
    echo "🌐 Repository URL: $REPO_URL"
    echo "📦 Clone command:"
    echo "   git clone $SSH_URL"
    echo ""
    echo "🚀 שלבים הבאים:"
    echo "   1. פתח Issues לדיווח באגים"
    echo "   2. הגדר GitHub Actions"
    echo "   3. צור Releases"
    echo "   4. שתף עם הקהילה!"
    
else
    echo "❌ שגיאה ביצירת repository"
    echo ""
    echo "📝 Response:"
    echo "$API_RESPONSE"
    echo ""
    echo "🔧 פתרון בעיות:"
    echo "   1. ודא שה-token תקין"
    echo "   2. בדוק שהרשאות נכונות"
    echo "   3. נסה שוב"
fi

# נקה את ה-token
rm -f /tmp/github_token_calclaw.txt

echo ""
echo "📚 משאבים:"
echo "   • GitHub: https://github.com/$GITHUB_USERNAME/$REPO_NAME"
echo "   • Actions: https://github.com/$GITHUB_USERNAME/$REPO_NAME/actions"
echo "   • Issues: https://github.com/$GITHUB_USERNAME/$REPO_NAME/issues"
echo ""
echo "🚀 Calclaw על GitHub! 🎉"