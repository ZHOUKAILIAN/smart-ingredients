# Project Requirements - Smart Ingredients

## Overview

Smart Ingredients is an intelligent food ingredient analysis tool that helps users quickly understand the healthiness of food products through image recognition and AI analysis.

## Core Features

### 1. Image Upload & OCR

**User Story**: As a user, I want to upload a photo of a food ingredient label so that I can analyze its contents.

**Requirements**:
- Support image capture from camera
- Support image upload from device storage
- Accept common image formats (JPEG, PNG, WebP)
- Maximum file size: 10MB
- Image preview before submission

**Acceptance Criteria**:
- [ ] User can capture photo using device camera
- [ ] User can select image from gallery
- [ ] Image preview is displayed before upload
- [ ] Invalid file types are rejected with clear error message
- [ ] File size validation prevents uploads > 10MB

### 2. OCR Recognition

**User Story**: As a user, I want the system to automatically extract text from the ingredient label image.

**Requirements**:
- Extract Chinese and English text from ingredient labels
- Handle various label layouts (vertical, horizontal, mixed)
- Maintain text structure and formatting
- Return confidence scores for each recognized segment

**Acceptance Criteria**:
- [ ] OCR returns structured text output
- [ ] Confidence scores are provided for validation
- [ ] Failed OCR attempts are logged and retried
- [ ] Processing time < 5 seconds for standard images

### 3. Ingredient Analysis

**User Story**: As a user, I want to receive a health analysis of the ingredients so I can make informed decisions.

**Requirements**:
- Identify potentially harmful additives
- Highlight allergens
- Provide health score (0-100)
- Generate detailed analysis report

**Analysis Categories**:
- **Additives**: Preservatives, colorings, flavorings
- **Allergens**: Common allergens (nuts, dairy, gluten, etc.)
- **Nutrition**: Sugar, sodium, fat content
- **Safety**: Banned or restricted ingredients

**Acceptance Criteria**:
- [ ] Health score is calculated and displayed
- [ ] Harmful ingredients are highlighted with warnings
- [ ] Allergens are clearly identified
- [ ] Detailed analysis includes ingredient explanations
- [ ] Analysis completes within 10 seconds

### 4. History & Favorites

**User Story**: As a user, I want to save and review my previous analyses.

**Requirements**:
- Save analysis history locally
- Mark products as favorites
- Search and filter history
- Export analysis results

**Acceptance Criteria**:
- [ ] History list shows past analyses with timestamps
- [ ] Users can mark items as favorites
- [ ] Search by product name or date
- [ ] Export results as PDF or shareable link

## Non-Functional Requirements

### Performance

- Image upload: < 2 seconds
- OCR processing: < 5 seconds
- LLM analysis: < 10 seconds
- Total response time: < 20 seconds

### Reliability

- System uptime: 99.5%
- OCR accuracy: > 90% for clear labels
- Analysis consistency: Same input produces same output

### Security

- Image data encrypted at rest
- User data stored locally (no cloud sync initially)
- API keys stored securely (environment variables)
- Rate limiting on public APIs

### Scalability

- Support 100+ concurrent users
- Handle 1000+ daily analysis requests
- Horizontal scaling capability for backend

## Technical Constraints

### Frontend

- Framework: Rust + Tauri + Leptos
- Platform: Desktop (macOS, Windows, Linux)
- Offline capability: Basic history access

### Backend

- Framework: Rust + Axum
- Database: PostgreSQL
- Cache: Redis
- API: RESTful

### External Services

- OCR: PaddleOCR (self-hosted) or Tesseract
- LLM: DeepSeek / 智谱 AI
- Storage: MinIO (local) / OSS (production)

## User Interface Requirements

### Design Principles

- Clean, intuitive interface
- Mobile-friendly layouts
- Clear visual feedback for actions
- Accessible color contrast

### Key Screens

1. **Home/Camera**: Camera capture and image selection
2. **Preview**: Image preview with upload confirmation
3. **Processing**: Loading states and progress indicators
4. **Results**: Health score, ingredient list, warnings
5. **History**: List of past analyses
6. **Settings**: User preferences and configuration

## Data Requirements

### Input Data

- Product image (JPEG/PNG/WebP)
- Optional product name (manual entry)

### Output Data

- Extracted text from image
- Health score (0-100)
- Ingredient analysis (categorized)
- Warnings and recommendations
- Timestamp and metadata

## Future Enhancements

- [ ] Barcode scanning
- [ ] Product database lookup
- [ ] Nutrition label analysis
- [ ] Multi-language support
- [ ] Cloud sync across devices
- [ ] Community ratings and reviews
