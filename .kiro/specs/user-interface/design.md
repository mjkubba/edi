# وثيقة التصميم

## نظرة عامة

تصميم واجهة مستخدم ويب حديثة ومتجاوبة لنظام معالجة ملفات EDI. الواجهة ستكون تطبيق ويب أحادي الصفحة (SPA) يوفر تجربة مستخدم سلسة لرفع ومعالجة وتحميل ملفات EDI دون الحاجة لاستخدام سطر الأوامر.

## البنية المعمارية

### البنية العامة
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Frontend      │    │   Backend API   │    │   EDI Core      │
│   (React/Vue)   │◄──►│   (Actix-web)   │◄──►│   (Rust Lib)    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### مكونات النظام

#### 1. الواجهة الأمامية (Frontend)
- **التقنية**: React.js مع TypeScript
- **التصميم**: Material-UI أو Tailwind CSS للتصميم المتجاوب
- **إدارة الحالة**: React Context API أو Zustand
- **رفع الملفات**: React Dropzone للسحب والإفلات
- **التواصل**: Axios للتواصل مع API

#### 2. الخادم الخلفي (Backend API)
- **التقنية**: Actix-web (Rust web framework)
- **معالجة الملفات**: Multipart form handling
- **التخزين المؤقت**: File system storage للملفات المرفوعة
- **قاعدة البيانات**: SQLite للتاريخ والبيانات الوصفية
- **المعالجة غير المتزامنة**: Tokio للمعالجة المتوازية

#### 3. مكتبة EDI الأساسية
- **الاستخدام**: المكتبة الحالية كما هي
- **التكامل**: Wrapper functions للاستدعاء من API

## المكونات والواجهات

### 1. مكونات الواجهة الأمامية

#### FileUploadComponent
```typescript
interface FileUploadProps {
  onFileSelect: (files: File[]) => void;
  acceptedFormats: string[];
  maxFileSize: number;
}
```

#### ProcessingStatusComponent
```typescript
interface ProcessingStatus {
  fileId: string;
  fileName: string;
  status: 'uploading' | 'processing' | 'completed' | 'error';
  progress: number;
  errorMessage?: string;
}
```

#### ResultsViewerComponent
```typescript
interface ProcessingResult {
  fileId: string;
  originalFileName: string;
  processedAt: Date;
  jsonResult: object;
  downloadUrl: string;
}
```

#### HistoryComponent
```typescript
interface HistoryEntry {
  id: string;
  fileName: string;
  processedAt: Date;
  status: string;
  fileSize: number;
  downloadUrl?: string;
}
```

### 2. واجهات API الخلفية

#### رفع الملفات
```rust
POST /api/upload
Content-Type: multipart/form-data

Response: {
  "file_id": "uuid",
  "status": "uploaded",
  "message": "File uploaded successfully"
}
```

#### حالة المعالجة
```rust
GET /api/status/{file_id}

Response: {
  "file_id": "uuid",
  "status": "processing|completed|error",
  "progress": 0-100,
  "error_message": "string|null"
}
```

#### تحميل النتائج
```rust
GET /api/download/{file_id}
Content-Type: application/json

Response: JSON file or error
```

#### التاريخ
```rust
GET /api/history

Response: {
  "entries": [
    {
      "id": "uuid",
      "file_name": "string",
      "processed_at": "ISO8601",
      "status": "string",
      "file_size": number
    }
  ]
}
```

## نماذج البيانات

### FileRecord (قاعدة البيانات)
```sql
CREATE TABLE file_records (
    id TEXT PRIMARY KEY,
    original_name TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    status TEXT NOT NULL,
    uploaded_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    processed_at DATETIME,
    error_message TEXT,
    result_path TEXT
);
```

### ProcessingJob (في الذاكرة)
```rust
#[derive(Debug, Clone)]
pub struct ProcessingJob {
    pub id: String,
    pub file_path: String,
    pub status: JobStatus,
    pub progress: u8,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone)]
pub enum JobStatus {
    Queued,
    Processing,
    Completed,
    Failed,
}
```

## معالجة الأخطاء

### أنواع الأخطاء
1. **أخطاء رفع الملفات**
   - حجم الملف كبير جداً
   - نوع الملف غير مدعوم
   - فشل في الرفع

2. **أخطاء المعالجة**
   - ملف EDI غير صحيح
   - تنسيق غير مدعوم
   - خطأ في التحليل

3. **أخطاء النظام**
   - نفاد مساحة التخزين
   - خطأ في قاعدة البيانات
   - خطأ في الخادم

### استراتيجية المعالجة
```typescript
interface ErrorResponse {
  error: {
    code: string;
    message: string;
    details?: any;
  };
}
```

## استراتيجية الاختبار

### 1. اختبارات الواجهة الأمامية
- **Unit Tests**: Jest + React Testing Library
- **Integration Tests**: Cypress للاختبارات الشاملة
- **Visual Tests**: Storybook للمكونات

### 2. اختبارات API
- **Unit Tests**: Rust built-in testing
- **Integration Tests**: Actix-web test utilities
- **Load Tests**: Artillery أو k6

### 3. اختبارات النظام الشامل
- **File Upload Tests**: اختبار رفع ملفات مختلفة الأحجام والأنواع
- **Processing Tests**: اختبار معالجة جميع أنواع EDI المدعومة
- **Error Handling Tests**: اختبار سيناريوهات الأخطاء المختلفة

## الأمان

### 1. رفع الملفات
- فحص نوع الملف (MIME type validation)
- تحديد حد أقصى لحجم الملف (10MB)
- فحص محتوى الملف للتأكد من أنه EDI صحيح

### 2. تخزين الملفات
- تشفير أسماء الملفات
- تنظيف الملفات القديمة تلقائياً
- عزل ملفات المستخدمين

### 3. API Security
- Rate limiting لمنع الإساءة
- CORS configuration
- Input validation وsanitization

## الأداء

### 1. تحسينات الواجهة الأمامية
- Code splitting للتحميل التدريجي
- Lazy loading للمكونات
- Caching للنتائج المعروضة

### 2. تحسينات الخادم
- معالجة متوازية للملفات
- Connection pooling لقاعدة البيانات
- File streaming للملفات الكبيرة

### 3. التخزين المؤقت
- Browser caching للأصول الثابتة
- Server-side caching للنتائج المتكررة
- CDN للملفات الثابتة (إذا لزم الأمر)

## النشر والتشغيل

### 1. بيئة التطوير
- Docker Compose للتطوير المحلي
- Hot reloading للواجهة الأمامية
- Auto-restart للخادم الخلفي

### 2. بيئة الإنتاج
- Docker containers
- Reverse proxy (Nginx)
- SSL/TLS certificates
- Monitoring وlogging

### 3. CI/CD Pipeline
- Automated testing
- Build optimization
- Deployment automation
- Health checks