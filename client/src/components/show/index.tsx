import { Link } from 'react-router-dom';
import './index.css';

function bytesToBase64(bytes: number[] | undefined): string {
    if (bytes == undefined) {
        return '';
    }

    const binString = Array.from(bytes, (x) => String.fromCodePoint(x)).join(
        ''
    );
    return btoa(binString);
}

interface Props {
    title: string;
    cover?: number[];
}

function Show(props: Props) {
    return (
        <Link to={`/shows/${props.title}`}>
            <div
                className='show'
                style={{
                    backgroundImage: `url(data:image/png;base64,${bytesToBase64(
                        props.cover
                    )})`,
                }}
                key={props.title}
            >
                <div className='show-overlay'>
                    <p className='show-title'>{props.title}</p>
                </div>
            </div>
        </Link>
    );
}

export default Show;
