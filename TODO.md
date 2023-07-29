- [] save video progress into an object inside of one key named video_progress
```typescript
    interface VideoProgress {
        [key: string]: {
            episodes: {video: number, progress: number}[],
            movies: {video: number, progress: number}[],
        }
    }
```
- [] remember video player volume
- [] animeList component: pass the list as a prop (allow it to be used in genres/studios pages)
- [] Parse episode title from the file name
- [] Setup relationships between anime

- [] NOT IMPORTANT!: Add mangas

Loader:
    - [] allow it to be callable from any were
    - [] reload only episodes or movies or openings or endings
    - [] refresh thumbnails

config:
    - [] add config file
    - [] add option to config thumbnails cache default to anime_directory/.sakugahd/thumbnails
    - [] add option to config db_path default anime_directory/.sakugahd/sakugahd.db
