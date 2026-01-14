# Define here the models for your scraped items
#
# See documentation in:
# https://docs.scrapy.org/en/latest/topics/items.html

import scrapy


class PriceCrawlerItem(scrapy.Item):
    # define the fields for your item here like:
    # name = scrapy.Field()
    pass

class RawHtmlItem(scrapy.Item): 
    name = scrapy.Field()
    url = scrapy.Field()
    html = scrapy.Field()
    createdAt = scrapy.Field()