export type AnimeFormat = 'TV' | 'OVA' | 'ONA' | 'MOVIE' | 'SPECIAL';
export type AnimeSeason = 'SPRING' | 'SUMMER' | 'FALL' | 'WINTER';
export type AnimeStatus = 'FINISHED' | 'ONGOING';

export interface AnimeType {
    title: string;
    description: String;
    format?: AnimeFormat;
    status?: AnimeStatus;
    season?: AnimeSeason;
    season_year?: number;
    cover: string;
}

export interface EpisodeType {
    id: string;
    anime_title: string;
    title: string;
    number: number;
    is_filler: boolean;
    thumbnail_file_name: string;
    file_name: string;
}

export interface MovieType {
    id: string;
    anime_title: string;
    title: string;
    number: number;
    watch_after: number;
    file_name: string;
    thumbnail_file_name: string;
}

export interface OpeningType {
    id: string;
    anime_title: string;
    title: string;
    number: number;
    file_name: string;
    thumbnail_file_name: string;
}

export interface EndingType {
    id: string;
    anime_title: string;
    title: string;
    number: number;
    file_name: string;
    thumbnail_file_name: string;
}
