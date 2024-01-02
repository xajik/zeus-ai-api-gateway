# Bruno 

## Setup 

 * Download from https://www.usebruno.com/

## Upload file

File upload is not supported by Bruno, there are workarounds

* Use Script 
* Use CURL

### Use Script

Follow guidelines: https://github.com/usebruno/bruno/issues/769 

* run `npm install`
* execute query

### 3rd part 

#### Google 

* Vision API / Dashbaord https://console.cloud.google.com/apis/dashboard?project=linear-listener-407603
  *  OCR features testing online: https://cloud.google.com/vision/docs/ocr#try_it 
* Palm2 https://makersuite.google.com/app/apikey


#### OpenAI 
* Vision https://platform.openai.com/docs/guides/vision 
* Completion https://platform.openai.com/docs/api-reference/chat/create 


### Use CURL

Upload file: 
```
curl -X POST \
  -H "Content-Type: multipart/form-data" \
  -F "file=@test.jpg" \
  http://localhost:3001/api/v1/completion/file
```
