# Ultimate AI API Gateway

<p align="center">
  <img src="./img/zeus.png" alt="alt text" width="500px"/>
</p>

Starting point for LLM POC project

## What's inside 

Project provides: 
 * Postrges DB 
    * PG Vector extension
    * Utils for migrations
    * PG Admin to browse the database
 * <b>ZEUS</b> - Rust, Actix-web server with:
    * LLM API: OpenAI, Gemini, Palm, Cloudflare 
        * Complettion, Visual & Embeddings 
    * Other API: Google Vision, Google Places
    * Connection with Postgress
 * <a href="https://www.usebruno.com/">Bruno</a> for testing APIs
 * <b>Athena</b> Python, Flask app
   * For everyting you cannot do in Rust 
   * Integrates Hugging Face library
      * Embedding routes 

### Database 

 * Preview at PGAdmin `http://localhost:8080/browser/` 
 * Connect with credentials from `.env` file
  * <p align="center">
    <img src="./img/pg-admin-properties.png" alt="alt text" width="300px"/>
    </p>

### Docker

 * Up    `docker-compose up -d`
 * Start `docker-compose start -d`
 * Stop  `docker-compose stop`
 * Down  `docker-compose down -v`
 * Logs  `docker-compose logs -f -t`

 If you are using `Colima` client:

 * Start `colima start`
    * In case you receive error `137` - add more RAM `colima start --memory 8`
 * Stop `colima stop`

 ### Propmpt 

 * Use `.prompt-generating-super-prompt.txt` to generate good prompt for LLM