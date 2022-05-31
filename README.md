
# Google Maps Scrapper Project

This is a learning project on exploring rust and web scraping. Contributios are to be welcomed.
Ps; this is only an education project and not more. Any request to take down this project will be immediately noticed.




## Contributing

I am freely open for any contributions to make the codes cleaner.

Add on, I am also open for any suggestions on how to make this scraping a lot more faster.

One major bugs that need to be solved is scrolling problem when scraping the google maps itself.
Unfortunately, the scrolling process when scraping need to be manual for now.



## Features

- multiple links can be scrapped 
- collect names
- collect phone-number (if exist)
- check if facebook exist


## Roadmap

- Add more infos to be collected
- collects fb link (if exist)
- collects address
- collects website link
- collects top reviews
- add settings on what to be collected in the UI 
- increase the scraping speed


## Usage

- Ready up the chromedriver. If your chrome's version is 102, you can directly use the chromedriver provided.
- Open 2 separate terminals (1 for server, 1 for client)

**SERVER TERMINAL**
- Use 1 of the terminal you open and run this command
```
cargo run -p scrapper

```
**CLIENT TERMINAL**
- Use another terminal to run this command
```
cargo run -p ui
```
- You will see an interface for the place to put your links.
- Just put all the links and divide them by just putting them in a different lines.
- Exe; 
```
https://www.google.com/maps/search/fashion+new+york/@40.6971494,-74.2598655,10z/data=!3m1!4b1
https://www.google.com/maps/search/restaurant+new+york/@40.7465286,-74.0027366,14z/data=!3m1!4b1
```
- click 'RUN'
- Be sure to manually scroll down when scraping the google maps.
- to check progress, click 'REFRESH'


