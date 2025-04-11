# FIRST TEST
# basic scraping of URLs


# import aiohttp
# import asyncio
# import time

# async def fetch(session, url):
#     async with session.get(url) as _:
#         pass

# async def main():
#     with open("urls.txt") as f:
#         urls = [line.strip() for line in f.readlines()]

#     start = time.time()
#     async with aiohttp.ClientSession() as session:
#         await asyncio.gather(*(fetch(session, url) for url in urls))
#     print("Python:", time.time() - start)

# asyncio.run(main())



# SECOND TEST
# includes scraping and word counting



import aiohttp
import asyncio
import time
from bs4 import BeautifulSoup

async def fetch(session, url):
    try:
        async with session.get(url) as resp:
            text = await resp.text()
            soup = BeautifulSoup(text, "html.parser")
            paragraphs = soup.find_all("p")
            content = " ".join(p.get_text(strip=True) for p in paragraphs)
            print(f"{url:<60} -> {len(content.split())} words")
    except Exception as e:
        print(f"Error fetching {url}: {e}")

async def main():
    with open("urls.txt") as f:
        urls = [line.strip() for line in f]

    start = time.time()
    async with aiohttp.ClientSession() as session:
        await asyncio.gather(*(fetch(session, url) for url in urls))

    print(f"Python Done in: {time.time() - start:.2f} seconds")

asyncio.run(main())