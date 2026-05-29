#!/bin/bash
set -e

echo "🚀 Starting deployment..."

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Step 1: Pull latest code
echo -e "${YELLOW}📥 Pulling latest code...${NC}"
git checkout main
git pull origin main

# Step 2: Build backend
echo -e "${YELLOW}🔨 Building backend...${NC}"
cd backend
cargo build --release
cd ..
echo -e "${GREEN}✅ Backend built successfully${NC}"

# Step 3: Build frontend
echo -e "${YELLOW}🔨 Building frontend...${NC}"
cd frontend
bun install
bun run build
cd ..
echo -e "${GREEN}✅ Frontend built successfully${NC}"

# Step 4: Deploy frontend to Alibaba
echo -e "${YELLOW}📤 Deploying frontend to Alibaba...${NC}"
scp -i ~/.ssh/alibabakey.pem -r frontend/build/* \
  ubuntu@47.84.137.49:/var/www/hermes-dashboard/
echo -e "${GREEN}✅ Frontend deployed${NC}"

# Step 5: Restart backend
echo -e "${YELLOW}🔄 Restarting backend service...${NC}"
sudo systemctl restart hermes-dashboard
echo -e "${GREEN}✅ Backend restarted${NC}"

# Step 6: Verify
echo -e "${YELLOW}🔍 Verifying deployment...${NC}"
sleep 2

BACKEND_STATUS=$(curl -s http://localhost:3001/api/health | jq -r '.status')
if [ "$BACKEND_STATUS" = "ok" ]; then
    echo -e "${GREEN}✅ Backend is healthy${NC}"
else
    echo -e "${RED}❌ Backend health check failed${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}🎉 Deployment complete!${NC}"
echo "   Frontend: http://47.84.137.49"
echo "   Backend:  http://43.156.247.129:3001"
