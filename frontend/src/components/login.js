import '../logreg'

import { Link, route } from "preact-router";
import { useState } from "preact/hooks";
import authService from "../services/auth.service";

export default function Login() {
    const [username, setUsername] = useState("");
    const [pass, setPass] = useState("");
    const [errorMsg, setErrorMsg] = useState("");

    const onUsernameInput = e => {
        setUsername(e.target.value);
    }

    const onPassInput = e => {
        setPass(e.target.value);
    }

    const onLogin = async e => {
        // console.log("logging in user: " + username + ", pass: " + pass);
        document.querySelector("#login_button").classList.add("is-loading");
        const res = await authService.login(username, pass);
        if (res.error) {
            setErrorMsg(res.error);
            document.querySelector("#login_button").classList.remove("is-loading");
        } else {
            route("/overview", true);
            var popStateEvent = new PopStateEvent('popstate', { state: null });
            dispatchEvent(popStateEvent);
        }
        // console.log(res);
    }

    return (
        <div class="logreg">
            <h3>Anmeldung</h3>
            <form class="logreg" onSubmit={onLogin}>
                <div class="field">
                    <input class="input" type="text" placeholder="Benutzername" value={username} onInput={onUsernameInput} />
                </div>

                <div class="field">
                    <input class="input" type="password" placeholder="Passwort" value={pass} onInput={onPassInput} />
                </div>
                <div class="has-text-danger">{errorMsg}</div>

                <button id="login_button" class="button is-success" onClick={onLogin} type="submit">Anmelden</button>
            </form>
            <div>
                Noch nicht registriert? <Link class="button" href="/register">Registrieren</Link>
            </div>
        </div>
    );
}