#%%  get the ids of the movies that were added to the database yesterday
import pandas as pd
import numpy as np
import requests
from datetime import datetime, timedelta

import os

API_KEY = os.getenv("TMDB_API_KEY")
yesterday_date = datetime.now() - timedelta(days=1)
# Format the date of yesterday as "dd_mm_yyyy"
date = yesterday_date.strftime("%m_%d_%Y")
#date = "01_03_2025"

url = "http://files.tmdb.org/p/exports/movie_ids_{}.json.gz".format(date)

headers = {
    "accept": "application/json",
    "Authorization": API_KEY
}

response = requests.get(url, headers=headers)

if response.status_code == 200 :
    file = "movie_ids.json.gz".format(date)
    with open(file, "wb") as f:
        f.write(response.content)
    
    # Read the gzip file into a DataFrame
    df = pd.read_json(file, lines=True)
    print(df.head())
else:
    print("Failed to download the file. Status code:", response.status_code)
    print("Content type:", response.headers.get('Content-Type'))
    print("Response content:", response.content[:100])

# %% find every genre in the database
from themoviedb import TMDb
import json

tmdb = TMDb(key=API_KEY, language="en", region="US")
# or: tmdb = aioTMDb(key="YOUR_API_KEY", language="pt-BR", region="BR")

movies = tmdb.movies().top_rated().results
movies = [vars(movie) for movie in movies]

genres = tmdb.genres().movie()

dict_genre = {genre.id: genre.name for genre in genres}

file = "genre_dict.json"
with open(file, "w") as f:
    json.dump(dict_genre, f)
# %%
