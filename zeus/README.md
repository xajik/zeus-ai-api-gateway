# Zeus 

API server for the AI API Gateway

### Run 

* Run debug `cargo watch -x run` or use Docker image

### Docker

 * Build: `docker build -t zeus-1 -f Dockerfile .`
 * Run `docker run -p 3001:3001 -v ./src:/app/src zeus-1:latest`

### Port

 * If port in use, find PID with `lsof -i tcp:3001`
 * relese `kill PID`

 ### API 

### LLM / GPT 

 * Cloudflare - https://developers.cloudflare.com/workers-ai/ 
 * OpenAI  - https://platform.openai.com/docs/
 * Gemini & Palm - https://makersuite.google.com/app/apikey

### Other 

 * Google Vision - https://console.cloud.google.com/vertex-ai
 * Google Places - 

 #### OpenAI

References

* https://platform.openai.com/docs/api-reference/introduction 
* https://platform.openai.com/docs/guides/vision
