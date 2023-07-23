import { Link } from 'react-router-dom';
import './index.css';

interface Props {
    title: string;
    cover: string;
}

function Show(props: Props) {
    return (
        <Link to={`/shows/${props.title}`}>
            <div
                className='show'
                style={{
                    backgroundImage: `url(/api/shows/${encodeURI(
                        props.title
                    )}/cover/${encodeURI(props.cover)})`,
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
