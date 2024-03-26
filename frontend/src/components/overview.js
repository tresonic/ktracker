import '../overview'

import { useEffect, useState } from "preact/hooks";
import userService from "../services/user.service";

export default function Overview() {
    const [score, setScore] = useState(0);
    const [overview, setOverview] = useState([]);
    const [loading, setLoading] = useState(true);

    const [input, setInput] = useState("");
    const [errorMsg, setErrorMsg] = useState("");


    useEffect(async () => {
        userService.getMeters().then(res => {
            setOverview(res.data.entries);
            const sum = res.data.entries.reduce((sum, entry) => sum + entry.meters, 0);
            setScore(Math.round(sum * 100) / 100);
            setLoading(false);
        });
    }, [loading]);

    const onInput = e => {
        setInput(e.target.value)
    }

    const onSave = async e => {
        const num = Math.round(parseFloat(input) * 100) / 100;
        if (num <= 0 || !num) {
            setInput("");
            return;
        }

        const res = await userService.createEntry(num);
        if (res.error) {
            setErrorMsg(res.error);
        } else {
            setLoading(true);
            setInput("");
        }
    }

    return (
        <>
            <div class="scoreview">{score} km</div>
            <div class="entrybox">
                Neuer Eintrag <br />
                <form class="entryline" onSubmit={e=>{e.preventDefault()}}>
                    <input class="inputkm" size="8" type="number" placeholder="Distanz (km)" min="0" value={input} onInput={onInput} />
                    <button id="save_button" class="savebutton" type="submit" onClick={onSave}>
                            <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-device-floppy" width="27" height="27" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                                <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                                <path d="M6 4h10l4 4v10a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2"></path>
                                <circle cx="12" cy="14" r="2"></circle>
                                <polyline points="14 4 14 8 8 8 8 4"></polyline>
                            </svg>
                    </button>
                </form>


                <div class="has-text-danger pb-3">{errorMsg}</div>
            </div>

            <table class="table mx-auto">
                <thead>
                    <tr>
                        <th>Zeit</th>
                        <th>Kilometer</th>
                    </tr>
                </thead>
                <tbody>
                    {overview.length > 0 && overview.map(entry => {
                        return (
                            <tr>
                                <td>{entry.time}</td>
                                <td>{entry.meters}</td>
                            </tr>
                        );
                    })}
                </tbody>
            </table>
        </>
    );
}