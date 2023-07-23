export type ShowFormat = 'TV' | 'OVA' | 'ONA' | 'MOVIE' | 'SPECIAL';
export type ShowSeason = 'SPRING' | 'SUMMER' | 'FALL' | 'WINTER';
export type ShowStatus = 'FINISHED' | 'ONGOING';

export interface ShowType {
    title: string;
    description: String;
    format?: ShowFormat;
    status?: ShowStatus;
    season?: ShowSeason;
    season_year?: number;
    cover: string;
}

export interface EpisodeType {
    id: string;
    show_title: string;
    title: string;
    number: number;
    is_filler: boolean;
    thumbnail_file_name: string;
    file_name: string;
}

export interface MovieType {
    id: string;
    show_title: string;
    title: string;
    number: number;
    watch_after: number;
    file_name: string;
    thumbnail_file_name: string;
}

export interface OpeningType {
    id: string;
    show_title: string;
    title: string;
    number: number;
    file_name: string;
    thumbnail_file_name: string;
}

export interface EndingType {
    id: string;
    show_title: string;
    title: string;
    number: number;
    file_name: string;
    thumbnail_file_name: string;
}
