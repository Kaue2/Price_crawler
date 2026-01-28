import scrapy
import uuid
from datetime import datetime
from price_crawler.items import RawHtmlItem

class BookSpider(scrapy.Spider):
    name = "books"
    start_urls = ["http://books.toscrape.com/catalogue/a-light-in-the-attic_1000/index.html"]

    def parse(self, response): 
        spider_uuid = str(uuid.uuid4())

        item = RawHtmlItem()
        item['_id'] = spider_uuid
        item['url'] = response.url
        item['html'] = response.text

        yield item
