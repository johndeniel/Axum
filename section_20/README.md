## Description

This endpoint allows you to update a task's information atomically in the database.

## Request

### Method

`PUT`

### URL

`http://localhost:3001/path/{id}`

### Example Request Body

```json
{
    "id": 123,
    "priority": "A",
    "title": "Updated",
    "description": "This is a sample Update.",
    "is_default": true
}
```