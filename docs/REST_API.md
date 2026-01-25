# AI-Todo REST API Documentation

**Base URL:** `http://localhost:3000`  
**Content-Type:** `application/json`

---

## Data Models

### Todo

| Field         | Type                | Required | Description                                |
|---------------|---------------------|----------|--------------------------------------------|
| `id`          | UUID                | Yes      | Unique identifier (auto-generated)         |
| `title`       | string              | Yes      | Todo title (min 1 character)               |
| `description` | string \| null      | No       | Optional description (max 500 characters)  |
| `status`      | TodoStatus          | Yes      | Current status of the todo                 |
| `priority`    | Priority            | Yes      | Priority level                             |
| `source`      | TodoSource          | Yes      | How the todo was created                   |
| `created_at`  | ISO 8601 datetime   | Yes      | Creation timestamp (UTC)                   |
| `updated_at`  | ISO 8601 datetime   | Yes      | Last update timestamp (UTC)                |

### Enums

#### TodoStatus
- `Todo` - Not started
- `Doing` - In progress
- `Done` - Completed

#### Priority
- `Low`
- `Medium` (default)
- `High`

#### TodoSource
- `Manual` - Created by user (default)
- `Audio` - Created from audio input
- `Ai` - Created by AI

---

## Endpoints

### Create Todo

**POST** `/todos`

Creates a new todo item.

**Request Body:**
```json
{
  "title": "string (required, min 1 char)",
  "description": "string (optional, max 500 chars)",
  "priority": "Low | Medium | High (optional, defaults to Medium)"
}
```

**Response:** `200 OK`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "Buy groceries",
  "description": "Milk, eggs, bread",
  "status": "Todo",
  "priority": "Medium",
  "source": "Manual",
  "created_at": "2026-01-22T23:17:30Z",
  "updated_at": "2026-01-22T23:17:30Z"
}
```

**Errors:**
- `400 Bad Request` - Validation failed

---

### List Todos

**GET** `/todos`

Returns all todos ordered by creation date (newest first).

**Response:** `200 OK`
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "Buy groceries",
    "description": "Milk, eggs, bread",
    "status": "Todo",
    "priority": "Medium",
    "source": "Manual",
    "created_at": "2026-01-22T23:17:30Z",
    "updated_at": "2026-01-22T23:17:30Z"
  }
]
```

---

### Get Todo

**GET** `/todos/:id`

Returns a single todo by ID.

**Path Parameters:**
- `id` (UUID) - The todo ID

**Response:** `200 OK`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "Buy groceries",
  "description": "Milk, eggs, bread",
  "status": "Todo",
  "priority": "Medium",
  "source": "Manual",
  "created_at": "2026-01-22T23:17:30Z",
  "updated_at": "2026-01-22T23:17:30Z"
}
```

**Errors:**
- `404 Not Found` - Todo not found

---

### Update Todo

**PATCH** `/todos/:id`

Partially updates a todo. Only provided fields are updated.

**Path Parameters:**
- `id` (UUID) - The todo ID

**Request Body:**
```json
{
  "title": "string (optional, min 1 char)",
  "description": "string (optional, max 500 chars)",
  "status": "Todo | Doing | Done (optional)",
  "priority": "Low | Medium | High (optional)"
}
```

**Response:** `200 OK`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "Buy groceries today",
  "description": "Milk, eggs, bread",
  "status": "Doing",
  "priority": "High",
  "source": "Manual",
  "created_at": "2026-01-22T23:17:30Z",
  "updated_at": "2026-01-22T23:20:00Z"
}
```

**Errors:**
- `400 Bad Request` - Validation failed
- `404 Not Found` - Todo not found

---

### Delete Todo

**DELETE** `/todos/:id`

Deletes a todo by ID.

**Path Parameters:**
- `id` (UUID) - The todo ID

**Response:** `200 OK` (empty body)

**Errors:**
- `404 Not Found` - Todo not found

---

---

### Suggest Tasks from Audio

**POST** `/audio/suggest`

Analyzes an audio file using **Gemini 2.5 Flash** to suggest potential tasks.

**Request Body:** `multipart/form-data`
- `audio`: The audio file (e.g., `.wav`, `.mp3`).

**Response:** `200 OK`
```json
{
  "tasks": [
    {
      "title": "Buy milk",
      "description": "2% or whole milk",
      "priority": "Medium"
    },
    {
      "title": "Call the dentist",
      "description": "Schedule a cleaning",
      "priority": "High"
    }
  ]
}
```

**Errors:**
- `400 Bad Request` - No audio data provided
- `500 Internal Server Error` - Gemini API failure or processing error

---

### Confirm Audio Tasks

**POST** `/audio/confirm`

Creates multiple todo items from a list of suggested task objects.

**Request Body:**
```json
{
  "tasks": [
    {
      "title": "Buy milk",
      "description": "2% or whole milk",
      "priority": "Medium"
    }
  ]
}
```

**Response:** `201 Created` (empty body)

**Errors:**
- `500 Internal Server Error` - Database insertion failure

---

## Error Response Format


All errors return a consistent JSON structure:

```json
{
  "message": "Error description",
  "status": 400,
  "errors": {
    "field_name": ["error message 1", "error message 2"]
  }
}
```

| HTTP Status | Description              |
|-------------|--------------------------|
| 400         | Validation failed        |
| 404         | Resource not found       |
| 500         | Internal server error    |
