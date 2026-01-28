import scrapy
import uuid
from price_crawler.items import RawHtmlItem
from scrapy_playwright.page import PageMethod

class KabumSpider(scrapy.Spider):
  name = "kabum"

  def start_requests(self):
    url = "https://www.kabum.com.br/hardware/placa-de-video-vga?page_number=1&page_size=20&facet_filters=&sort=most_searched"

    yield scrapy.Request(
      url,
      meta={
        "playwright": True,
        "playwright_page_methods": [
          PageMethod("wait_for_selector", "main a[href*='/produto/']")
        ],
                
        "playwright_include_page": True,
      },
      callback=self.parse
    )

  
  async def parse(self, response):
    product_links = response.css('main a[href*="/produto/"]::attr(href)').getall()
    product_links = list(set([link for link in product_links if "/produto/" in link]))

    print(f"DEBUG: {len(product_links)} foram encontrados")

    for link in product_links:
      url_completa = response.urljoin(link)

      yield scrapy.Request(
        url_completa,
        meta={
          "playwright": True,
          "playwright_page_methods": [
            PageMethod("wait_for_selector", "h1")
          ]
        },
        callback=self.parse_product
      )

  async def parse_product(self, response):
    file_name = "debug_kabum.html"
    with open(file_name, "wb") as f:
      f.write(response.body)

    print(response.url)
    spider_uuid = str(uuid.uuid4())

    item = RawHtmlItem()
    item['_id'] = spider_uuid
    item['url'] = response.url
    item['html'] = response.text

    yield item