import { ReactElement } from "react";
import CreateForm from "./uploadData";
import styles from "./upload.module.css";


export default function UploadDesc(): ReactElement {
    return (
        <main className={styles.uploader}>
            <h2>Input new listing</h2>
            <CreateForm/>
        </main>
    )
}
