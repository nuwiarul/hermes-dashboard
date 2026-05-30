#!/bin/bash
set -e

# Deploy Frontend to Alibaba Server
# Usage: ./scripts/deploy-frontend.sh

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

ALIBABA_HOST="47.84.137.49"
ALIBABA_USER="ubuntu"
ALIBABA_KEY="$HOME/.ssh/alibabakey.pem"
REMOTE_DIR="/var/www/hermes-dashboard"

echo -e "${YELLOW}🚀 Deploying frontend to Alibaba...${NC}"

# Step 1: Build frontend
echo -e "${YELLOW}📦 Building frontend...${NC}"
cd frontend
npm run build
cd ..
echo -e "${GREEN}✅ Frontend built${NC}"

# Step 2: Deploy to Alibaba
echo -e "${YELLOW}📤 Uploading to Alibaba...${NC}"
scp -i "$ALIBABA_KEY" -r frontend/build/* "$ALIBABA_USER@$ALIBABA_HOST:$REMOTE_DIR/"
echo -e "${GREEN}✅ Files uploaded${NC}"

# Step 3: Fix permissions on Alibaba
echo -e "${YELLOW}🔧 Fixing permissions...${NC}"
ssh -i "$ALIBABA_KEY" "$ALIBABA_USER@$ALIBABA_HOST" "sudo chown -R www-data:www-data $REMOTE_DIR"
echo -e "${GREEN}✅ Permissions fixed${NC}"

echo ""
echo -e "${GREEN}🎉 Frontend deployed successfully!${NC}"
echo "   URL: https://hermes.vinrul.my.id"
