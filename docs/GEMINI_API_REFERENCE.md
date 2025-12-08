# Gemini API Reference for Claude

This document summarizes the capabilities, models, and usage patterns of the Google Gemini API based on the provided documentation. Use this as a reference when generating code or answering questions about the Gemini API.

## 1. Models

| Model ID | Description | Key Features |
| :--- | :--- | :--- |
| **`gemini-3-pro-preview`** | Most intelligent model. | Agentic, "Thinking" process, 1M context, Multimodal (Text, Image, Video, Audio, PDF). |
| **`gemini-2.5-flash`** | Price/Performance leader. | Low latency, High volume, Multimodal. |
| **`gemini-3-pro-image-preview`** | Image generation. | Text/Image to Image. |
| **`veo-3.1-generate-preview`** | Video generation. | Text to Video (1080p, 8s). |
| **`gemini-embedding-001`** | Embeddings. | Text embeddings for RAG/Search. |

## 2. Core Capabilities

### 🧠 Reasoning & Thinking
- **Thinking Process**: Gemini 3 and 2.5 models use an internal chain of thought.
- **Configuration**: Set `thinking_level` to `"low"` (faster) or `"high"` (better reasoning).
- **Thought Signatures**: Encrypted tokens representing the thought process. **CRITICAL**: Must be passed back to the model in multi-turn conversations (especially for Function Calling).

### 👁️ Vision (Images & Video)
- **Input**:
  - **Inline**: Base64 encoded data (for small files < 20MB).
  - **File API**: Upload via `files.upload` (for large files > 20MB).
- **Media Resolution**: Control token usage vs. quality.
  - Levels: `media_resolution_low`, `media_resolution_medium`, `media_resolution_high`.
  - Can be set globally or per-part (Gemini 3 only).
- **PDF Processing**: Native understanding of text and visuals in PDFs (up to 1000 pages).

### 🗣️ Audio & Speech
- **Audio Understanding**: Analyze audio files (summary, transcription).
- **Text-to-Speech (TTS)**: Native generation.
  - Config: `response_modalities=["AUDIO"]`, `speech_config` with `voice_config` (select voice like 'Kore').

### 🛠️ Structured Output & Tools
- **JSON Schema**: Enforce output structure.
  - Config: `response_mime_type: "application/json"`, `response_json_schema: <schema>`.
  - SDK Support: Pydantic (Python), Zod (JS).
- **Function Calling**: Model returns tool calls; Client executes and returns results.
  - Supports **Parallel** and **Sequential** calls.

## 3. API Usage Patterns

### Authentication
- **Header**: `x-goog-api-key: $GEMINI_API_KEY`

### File API
- **Purpose**: Handle large media files (Images, Video, Audio, PDF).
- **Workflow**:
  1.  Upload: `client.files.upload(file=...)` -> Returns `File` object with URI.
  2.  Wait: Ensure state is `ACTIVE` (processing complete).
  3.  Use: Pass `file.uri` in `generateContent` request.
  4.  Delete: `client.files.delete(name=...)` when done (files expire after 48h).

### Caching
- **Implicit**: Auto-enabled for Gemini 2.5 when context matches. No setup needed.
- **Explicit**: Manually create cache for guaranteed cost savings on repeated large contexts.
  - **TTL**: Default 1 hour.
  - **Usage**: Pass `cached_content` in `generateContent` config.

### Batch API
- **Purpose**: Non-urgent, high-volume processing at **50% cost**.
- **Input**: Inline list of requests OR JSONL file (recommended for large batches).
- **Output**: JSONL file with results.

## 4. Prompt Engineering Best Practices

- **Clear Instructions**: Differentiate between "Question", "Task", "Entity", "Completion".
- **Few-Shot Prompting**: Provide 2-5 examples (Input -> Output) to guide style and format.
- **Context**: Provide background info (e.g., "You are a coding expert...").
- **Constraints**: Explicitly state what *not* to do.
- **Format**: Specify desired output format (JSON, Markdown, Table).

## 5. Code Snippets (Python `google-genai`)

### Basic Text Generation
```python
from google import genai
client = genai.Client(api_key="GEMINI_API_KEY")
response = client.models.generate_content(
    model="gemini-2.5-flash",
    contents="Explain quantum computing."
)
print(response.text)
```

### Multimodal (Image + Text)
```python
from google import genai
from google.genai import types
import PIL.Image

client = genai.Client()
image = PIL.Image.open("image.jpg")
response = client.models.generate_content(
    model="gemini-2.5-flash",
    contents=["Describe this image", image]
)
```

### Structured Output (Pydantic)
```python
from pydantic import BaseModel
class Recipe(BaseModel):
    name: str
    ingredients: list[str]

response = client.models.generate_content(
    model="gemini-2.5-flash",
    contents="Cookie recipe",
    config={
        "response_mime_type": "application/json",
        "response_json_schema": Recipe.model_json_schema(),
    }
)
```
