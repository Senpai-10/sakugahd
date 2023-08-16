import { PageType } from '../types';
import '../css/components/readingProgressbar.css';
import classNames from 'classnames';

interface Props {
    pages: PageType[];
    cursor: number;
    setCursor: any;
}

export function ReadingProgressbar(props: Props) {
    return (
        <div className='reading-progressbar'>
            {props.pages.map((page: PageType) => {
                const styles = classNames({
                    'reading-progressbar-block': true,
                    'reading-progressbar-block-active':
                        page.number - 1 <= props.cursor,
                    'reading-progressbar-done':
                        props.cursor == props.pages.length - 1,
                });
                return (
                    <div
                        key={page.id}
                        title={`page ${page.number}`}
                        onClick={() => props.setCursor(page.number - 1)}
                        className={styles}
                    ></div>
                );
            })}
        </div>
    );
}
