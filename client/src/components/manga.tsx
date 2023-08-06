import { Link } from 'react-router-dom';
import '/public/css/components/manga.css';

interface Props {
    title: string;
    cover?: string;
}

export function Manga(props: Props) {
    let imageUrl = () => {
        if (props.cover == undefined) {
            return '/default_cover.svg';
        }

        return `/api/manga/${encodeURI(props.title)}/cover/${encodeURI(
            props.cover
        )}`;
    };

    return (
        <Link to={`/manga/${props.title}`}>
            <div
                className='manga'
                style={{
                    backgroundImage: `url(${imageUrl()})`,
                }}
                key={props.title}
            >
                <div className='manga-overlay'>
                    <p className='manga-title'>{props.title}</p>
                </div>
            </div>
        </Link>
    );
}

