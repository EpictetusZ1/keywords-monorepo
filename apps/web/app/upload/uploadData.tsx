"use client"
import { FormEvent, useState } from "react"
import styles from "./upload.module.css"
import axios from "axios"


export default function CreateForm() {
    const [title, setTitle] = useState("")
    const [desc, setDesc] = useState("")
    const [req, setReq] = useState("")
    const [skills, setSkills] = useState("")

    const [isLoading, setIsLoading] = useState(false)

    const handleSubmit = async (e: FormEvent<HTMLFormElement>)  => {
        e.preventDefault()
        setIsLoading(true)


        const postingData = {
            title,
            desc,
            req,
            skills
        }

        // const res = await fetch('http://localhost:8080/', {
        //     method: "POST",
        //     headers: {"Content-Type": "application/json"},
        //     body: JSON.stringify(postingData)
        // })

        const data = await axios.post("http://127.0.0.1:8080/submit", postingData)
        console.log(data)
        setIsLoading(false)
    }

    return (
        <form onSubmit={handleSubmit} className={styles.formCont}>
            <label className={styles.labels}>
               Position Title:
                <input
                    required
                    type="text"
                    onChange={(e) => setTitle(e.target.value)}
                    value={title}
                />
            </label>
            <label className={styles.labels}>
               Description:
                <textarea
                    required
                    rows={10}
                    cols={75}
                    onChange={(e) => setDesc(e.target.value)}
                    value={desc}
                />
            </label>
            <label className={styles.labels}>
               Requirements:
                <textarea
                    required
                    rows={5}
                    cols={75}
                    onChange={(e) => setReq(e.target.value)}
                    value={req}
                />
            </label>
            <label className={styles.labels}>
                Skills:
                <textarea
                    required
                    rows={5}
                    cols={75}
                    onChange={(e) => setSkills(e.target.value)}
                    value={skills}
                />
            </label>
            <button disabled={isLoading}>
                {isLoading && <span>Adding...</span>}
                {!isLoading && <span>Add Desc</span>}
            </button>
        </form>
    )
}