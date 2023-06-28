import requests
import time

class Brand:
    def __init__(self, id, title, slug, favourite_count, pretty_favourite_count, item_count, pretty_item_count, is_visible_in_listings, path, requires_authenticity_check, is_luxury, url, is_favourite):
        self.id = id
        self.title = title
        self.slug = slug
        self.favourite_count = favourite_count
        self.pretty_favourite_count = pretty_favourite_count
        self.item_count = item_count
        self.pretty_item_count = pretty_item_count
        self.is_visible_in_listings = is_visible_in_listings
        self.path = path
        self.requires_authenticity_check = requires_authenticity_check
        self.is_luxury = is_luxury
        self.url = url
        self.is_favourite = is_favourite

class Brands:
    def __init__(self, brands, current_page, total_pages, total_entries, per_page, time):
        self.brands = brands
        self.pagination = {
            "current_page": current_page,
            "total_pages": total_pages,
            "total_entries": total_entries,
            "per_page": per_page,
            "time": time
        }
        self.code = 0


def get_data(url, sess):
    response = sess.get(url)
    if response.status_code == 200:
        data = response.json()
        return data
    else:
        return None

def deserialize_json(data):
    brands = []
    for brand in data["brands"]:
        deserialized_brand = Brand(
            id=brand["id"],
            title=brand["title"],
            slug=brand["slug"],
            favourite_count=brand["favourite_count"],
            pretty_favourite_count=brand["pretty_favourite_count"],
            item_count=brand["item_count"],
            pretty_item_count=brand["pretty_item_count"],
            is_visible_in_listings=brand["is_visible_in_listings"],
            path=brand["path"],
            requires_authenticity_check=brand["requires_authenticity_check"],
            is_luxury=brand["is_luxury"],
            url=brand["url"],
            is_favourite=brand["is_favourite"]
        )
        brands.append(deserialized_brand)

    deserialized_data = Brands(
        brands=brands,
        current_page=data["pagination"]["current_page"],
        total_pages=data["pagination"]["total_pages"],
        total_entries=data["pagination"]["total_entries"],
        per_page=data["pagination"]["per_page"],
        time=data["pagination"]["time"]
    )

    return deserialized_data



if __name__ == "__main__":
    
    file = open("nombres_marcas.txt" , "r")
    
    base_url = "https://www.vinted.es/api/v2/brands?keyword="
    # URL para hacer la petición

    url_cookies = 'https://www.vinted.es/auth/token_refresh'

    sess = requests.Session()
    
    HEADERS = {
                "User-Agent": "PostmanRuntime/7.28.4",  # random.choice(USER_AGENTS),
                "Host": "www.vinted.es",
    }

    sess.headers.update(HEADERS)

    sess.post(url_cookies)
    for brand in file:
    
        url = base_url+brand

        # Obtener los datos
        data = get_data(url, sess)

        if data is not None:
            # Serializar el JSON
            deserialized_data = deserialize_json(data)
            # Imprimir los resultados deserializados
            if len(deserialized_data.brands) > 0:
                brand = deserialized_data.brands[0]
                print(f"{brand.id} , {brand.title} , {brand.url}")
        else:
            print("Error al obtener los datos.")

        time.sleep(0.250)