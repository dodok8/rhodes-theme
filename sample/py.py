# Python example

"""
docstring
"""

import requests

def get_posts_by_user(user_id):
    response = requests.get(f'https://jsonplaceholder.typicode.com/users/{user_id}/posts')
    posts = response.json()

    for post in posts:
        print(post['title'])

def main():
    user_id = 1
    get_posts_by_user(user_id)

if __name__ == "__main__":
    main()