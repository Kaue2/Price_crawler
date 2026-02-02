import scrapy
import uuid
from price_crawler.items import RawHtmlItem
from scrapy_playwright.page import PageMethod

class KabumSpider(scrapy.Spider):
  name = "kabum"

  start_urls = ["https://www.kabum.com.br/produto/934759/console-sony-playstation-5-com-leitor-de-discos-ssd-1tb-controle-sem-fio-dualsense-2-jogos"]

  def start_requests(self):
    target_url = getattr(self, 'target_url', None)

    if target_url:
      yield scrapy.Request(
        url=url,
        meta={
          "playwright": True,
          "playwright_page_methods": [
            PageMethod("wait_for_selector", "h1")
          ]
        },
        callback=self.parse_product
      )

    else:
      if hasattr(self, 'start_urls'):
        for url in self.start_urls:
          yield scrapy.Request(
            url=url,
            meta={
              "playwright": True,
              "playwright_page_methods": [
                PageMethod("wait_for_selector", "h1")
              ]
            },
            callback=self.parse_product
          )

      else: 
        self.logger.warning("nenhuma url fornecida!")

  
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