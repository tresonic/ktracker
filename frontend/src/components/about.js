import { Link } from "preact-router";

export default function About() {
    return (
        <div style="text-align: center; width: 65vw;">
            <h2>Hallo bei der Fahrradchallenge!</h2>
            Hier sollen bis Ende Mai so viele gefahrene Fahrradkilometer wie möglich eingetragen werden.<br/>
            Für die Motivation kann man nach Anmeldung eine Highscore-Liste einsehen.<br/>
            <Link href="/register">Hier geht's zur Registrierung.</Link>
        </div>
    )
}