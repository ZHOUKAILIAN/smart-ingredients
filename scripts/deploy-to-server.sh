#!/bin/bash

# 部署配置（从 .env 读取，避免写死在仓库）
PROJECT_NAME="smart-ingredients"

# 颜色输出
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}开始部署 Smart Ingredients 到服务器${NC}"
echo -e "${GREEN}========================================${NC}"

# 检查 .env 文件
if [ ! -f .env ]; then
    echo -e "${RED}错误: .env 文件不存在${NC}"
    echo -e "${YELLOW}请先复制 .env.example 为 .env 并配置好环境变量${NC}"
    exit 1
fi

# 读取部署配置
set -a
source .env
set +a

if [ -z "${DEPLOY_SERVER_IP}" ] || [ -z "${DEPLOY_SERVER_USER}" ] || [ -z "${DEPLOY_REMOTE_DIR}" ]; then
    echo -e "${RED}错误: 部署配置缺失${NC}"
    echo -e "${YELLOW}请在 .env 中设置 DEPLOY_SERVER_IP / DEPLOY_SERVER_USER / DEPLOY_REMOTE_DIR${NC}"
    exit 1
fi

# 1. 创建临时目录并复制需要的文件
echo -e "${YELLOW}[1/6] 准备部署文件...${NC}"
TEMP_DIR=$(mktemp -d)
echo "临时目录: $TEMP_DIR"

# 复制必要的文件和目录
cp -r backend "$TEMP_DIR/"
cp -r shared "$TEMP_DIR/"
cp -r ocr_service "$TEMP_DIR/"
cp Cargo.toml "$TEMP_DIR/"
cp Cargo.lock "$TEMP_DIR/"
cp docker-compose.yml "$TEMP_DIR/"
cp .dockerignore "$TEMP_DIR/"
cp .env "$TEMP_DIR/"

echo -e "${GREEN}✓ 文件准备完成${NC}"

# 2. 创建部署包
echo -e "${YELLOW}[2/6] 创建部署包...${NC}"
DEPLOY_PACKAGE="/tmp/${PROJECT_NAME}-deploy.tar.gz"
tar -czf "$DEPLOY_PACKAGE" -C "$TEMP_DIR" .
echo -e "${GREEN}✓ 部署包创建完成: $DEPLOY_PACKAGE${NC}"

# 3. 上传到服务器
echo -e "${YELLOW}[3/6] 上传文件到服务器...${NC}"
ssh ${DEPLOY_SERVER_USER}@${DEPLOY_SERVER_IP} "mkdir -p ${DEPLOY_REMOTE_DIR}"
scp "$DEPLOY_PACKAGE" ${DEPLOY_SERVER_USER}@${DEPLOY_SERVER_IP}:/tmp/
echo -e "${GREEN}✓ 文件上传完成${NC}"

# 4. 在服务器上解压并安装 Docker
echo -e "${YELLOW}[4/6] 在服务器上配置环境...${NC}"
ssh ${DEPLOY_SERVER_USER}@${DEPLOY_SERVER_IP} << ENDSSH
set -e

# 解压部署包
cd "${DEPLOY_REMOTE_DIR}"
tar -xzf /tmp/smart-ingredients-deploy.tar.gz
rm /tmp/smart-ingredients-deploy.tar.gz

# 检查 Docker 是否安装
if ! command -v docker &> /dev/null; then
    echo "Docker 未安装，开始安装..."
    # 更新包索引
    apt-get update
    # 安装依赖
    apt-get install -y ca-certificates curl gnupg lsb-release
    # 添加 Docker 官方 GPG key
    mkdir -p /etc/apt/keyrings
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg
    # 设置仓库
    echo \
      "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
      $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null
    # 安装 Docker Engine
    apt-get update
    apt-get install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin
    echo "Docker 安装完成"
else
    echo "Docker 已安装"
fi

# 检查 Docker Compose 是否可用
if ! docker compose version &> /dev/null; then
    echo "Docker Compose 不可用，尝试安装..."
    apt-get update
    apt-get install -y docker-compose-plugin
fi

echo "环境配置完成"
ENDSSH

echo -e "${GREEN}✓ 服务器环境配置完成${NC}"

# 5. 停止旧容器并启动新容器
echo -e "${YELLOW}[5/6] 部署 Docker 容器...${NC}"
ssh ${DEPLOY_SERVER_USER}@${DEPLOY_SERVER_IP} << ENDSSH
set -e
cd "${DEPLOY_REMOTE_DIR}"

# 停止并删除旧容器
echo "停止旧容器..."
docker compose down || true

# 清理旧镜像（可选）
echo "清理未使用的镜像..."
docker image prune -f || true

# 构建并启动容器
echo "构建并启动新容器..."
docker compose up -d --build

# 等待服务启动
echo "等待服务启动..."
sleep 10

# 检查容器状态
echo "检查容器状态..."
docker compose ps

# 查看后端日志
echo "后端服务日志（最近20行）:"
docker compose logs --tail=20 backend

ENDSSH

echo -e "${GREEN}✓ Docker 容器部署完成${NC}"

# 6. 清理临时文件
echo -e "${YELLOW}[6/6] 清理临时文件...${NC}"
rm -rf "$TEMP_DIR"
rm -f "$DEPLOY_PACKAGE"
echo -e "${GREEN}✓ 清理完成${NC}"

# 显示部署信息
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}部署完成！${NC}"
echo -e "${GREEN}========================================${NC}"
echo -e "服务器地址: ${YELLOW}http://${DEPLOY_SERVER_IP}:3000${NC}"
echo -e "数据库端口: ${YELLOW}${DEPLOY_SERVER_IP}:5432${NC}"
echo -e "Redis 端口: ${YELLOW}${DEPLOY_SERVER_IP}:6379${NC}"
echo -e "MinIO 控制台: ${YELLOW}http://${DEPLOY_SERVER_IP}:9001${NC}"
echo ""
echo -e "查看日志命令:"
echo -e "${YELLOW}ssh ${DEPLOY_SERVER_USER}@${DEPLOY_SERVER_IP} 'cd ${DEPLOY_REMOTE_DIR} && docker compose logs -f backend'${NC}"
echo ""
echo -e "重启服务命令:"
echo -e "${YELLOW}ssh ${DEPLOY_SERVER_USER}@${DEPLOY_SERVER_IP} 'cd ${DEPLOY_REMOTE_DIR} && docker compose restart backend'${NC}"
echo ""
echo -e "停止服务命令:"
echo -e "${YELLOW}ssh ${DEPLOY_SERVER_USER}@${DEPLOY_SERVER_IP} 'cd ${DEPLOY_REMOTE_DIR} && docker compose down'${NC}"
