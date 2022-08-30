import { useEffect, useState } from "preact/hooks";
import userService from "../services/user.service";

export default function Highscore() {
    const [highscore, setHighscore] = useState([]);
    const [loading, setLoading] = useState(true);


    useEffect(async () => {
        userService.getHighscore().then(res => {
            setHighscore(res.data.entries);
            setLoading(false);
        });
    }, []);

    console.log(highscore);

    return (
        <div class="container mt-6">
            <div class="columns is-centered">
                <table class="table mx-auto">
                    <thead>
                        <tr>
                            <th>Platz</th>
                            <th>Benutzername</th>
                            <th>Kilometer</th>
                        </tr>
                    </thead>
                    <tbody>
                        {highscore.length > 0 && highscore.map((entry, idx) => {
                            return (
                                <tr>
                                    <td>{idx+1}</td>
                                    <td>{entry.username}</td>
                                    <td>{entry.meters}</td>
                                </tr>
                            );
                        })}
                    </tbody>
                </table>
            </div>
        </div>
    );
}