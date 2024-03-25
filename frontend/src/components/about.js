import { Link } from "preact-router";

export default function About() {
    return (
        <div style="text-align: center; width: 65vw;">
            <h2>Hallo bei der Fahrradchallenge!</h2>
            Hier sollen bis zum 15.5. so viele gefahrene Fahrradkilometer wie möglich eingetragen werden.<br/>
            <strong>Auf die Gewinner warten Preise!</strong><br/>
            Für die Motivation kann man nach Anmeldung eine Highscore-Liste einsehen.<br/>
            <Link href="/register">Hier geht's zur Registrierung.</Link>
        </div>
    )
}