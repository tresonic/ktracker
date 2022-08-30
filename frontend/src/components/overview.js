import { useEffect, useState } from "preact/hooks";
import authService from "../services/auth.service";
import userService from "../services/user.service";

export default function Overview() {
    const [overview, setOverview] = useState([]);
    const [loading, setLoading] = useState(true);

    const [input, setInput] = useState("");
    const [errorMsg, setErrorMsg] = useState("");


    useEffect(async () => {
        // const res = await userService.getHighscore();
        // console.log(res.data.entries);
        // setHighscore(res.data.entries);
        // console.log(highscore);
        userService.getMeters().then(res => {
            console.log(res);
            setOverview(res.data.entries);
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
        document.querySelector("#save_button").classList.add("is-loading");
        const res = await userService.createEntry(num);
        if (res.error) {
            setErrorMsg(res.error);
        } else {
            setLoading(true);
        }
        document.querySelector("#save_button").classList.remove("is-loading");
    }

    // if (loading) {
    //     return (<div>Overview loading...</div>)
    // }

    return (
        <div class="container mt-6">
            <div class="columns is-centered">
                <div class="column mx-auto">
                    <div class="has-text-centered">
                    {/* <label>Neuer Eintrag</label> */}
                    <div class="field is-horizontal is-grouped is-grouped-centered">
                        <div class="control">
                            <input class="input is-success" type="number" placeholder="Distanz in Kilometern" min="0" value={input} onInput={onInput}/>
                        </div>
                        <div class="control">
                            <a id="save_button" class="button is-info" onClick={onSave}>
                                Neuer Eintrag
                            </a>
                        </div>
                    </div>
                    

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
                        {/* <div>{highscore[0].username}</div> */}
                    </table>
                </div>
            </div>
        </div>
    );
}