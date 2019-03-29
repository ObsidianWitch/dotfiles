#!/usr/bin/env python3

import os
import argparse
import textwrap
import lxml.html
import requests

class Downloader:
    def __init__(self, start_url, elem_xpath, next_xpath):
        self.start_url  = start_url
        self.elem_xpath = elem_xpath
        self.next_xpath = next_xpath

    def start(self):
        for i, elem_url in enumerate(self.iterate()):
            self.download(
                url      = elem_url,
                filename = f"{i}_{os.path.basename(elem_url)}",
            )

    def request(self, url): return requests.get(
        url     = url,
        headers = { "user-agent": "Mozilla/5.0" },
    )

    def download(self, url, filename):
        print(url)
        with open(filename, "wb") as file:
            elem = self.request(url).content
            file.write(elem)

    def iterate(self):
        next_url = self.start_url
        while next_url:
            html = self.request(next_url).content
            tree = lxml.html.fromstring(html)

            next_url = tree.xpath(self.next_xpath) if self.next_xpath else None
            next_url = next_url[0] if next_url else None

            elem_urls = tree.xpath(self.elem_xpath)
            for elem_url in elem_urls: yield elem_url

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        formatter_class = argparse.RawDescriptionHelpFormatter,
        description = "Download elements from a sequence of webpages.",
        epilog = textwrap.dedent(f"""\
            examples:
                # Sleepless Domain
                xpathdl 'http://www.sleeplessdomain.com/comic/chapter-1-cover' \\
                        "//img[@id = 'cc-comic']/@src" \\
                        "//a[@rel = 'next']/@href"

                # The Night Belongs to Us
                xpathdl 'https://tnbtu.com/comic/01-00/' \\
                        "//div[@id = 'comic']//img/@src" \\
                        "//a[contains(@class, 'comic-nav-next')]/@href"

                # Windrose
                xpathdl 'https://sparklermonthly.com/wr/windrose-chapter-01-page-001/' \\
                        "//div[@class = 'webcomic-image']//img/@src" \\
                        "//a[@rel = 'next' and not(contains(@class, 'current-webcomic'))]/@href"

                # Never Satisfied
                xpathdl 'http://www.neversatisfiedcomic.com/comic/never-satisfied' \\
                        "//div[@id='cc-comicbody']//img/@src" \\
                        "//div[@id='cc-comicbody']/a/@href"
        """),
    )
    parser.add_argument("start_url",
        help = "first url to visit",
    )
    parser.add_argument("elem_xpath",
        help = "path expression to get the desired element(s)"
    )
    parser.add_argument("next_xpath",
        help  = "(optional) path expression to get the next page in the sequence",
        nargs = "?",
    )
    args = parser.parse_args()
    Downloader(args.start_url, args.elem_xpath, args.next_xpath).start()