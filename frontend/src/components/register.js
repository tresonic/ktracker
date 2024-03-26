import '../logreg'

import { Link, route } from "preact-router";
import { useState } from "preact/hooks";
import authService from "../services/auth.service";

export default function Register() {
    const [username, setUsername] = useState("");
    const [pass, setPass] = useState("");
    const [email, setEmail] = useState("");

    const [errorMsg, setErrorMsg] = useState("");

    const onUsernameInput = e => {
        setUsername(e.target.value);
    }

    const onPassInput = e => {
        setPass(e.target.value);
    }

    const onEmailInput = e => {
        setEmail(e.target.value);
    }

    const onRegister = async e => {
        if(!username || !pass || !email) {
            return;
        }
        document.querySelector("#register_button").classList.add("is-loading");
        const res = await authService.register(username, email, pass);
        console.log(res);
        if (res.error) {
            setErrorMsg(res.error);
            document.querySelector("#register_button").classList.remove("is-loading");
        } else {
            route("/login", true);
        }
    }

    return (
        <div class="logreg">
            <h3>Registrierung</h3>
            <form class="logreg" onSubmit={e=>{e.preventDefault()}}>
                <input class="input" type="text" placeholder="Benutzername" value={username} onInput={onUsernameInput}/>

                <input class="input" type="email" placeholder="E-Mail" value={email} onInput={onEmailInput}/>
                <div class="emaildisclaimer">Die E-Mail-Adresse wird <strong>nur</strong> verwendet, um den Gewinner zu kontaktieren.</div>
                <input class="input" type="password" placeholder="Passwort" value={pass} onInput={onPassInput}/>
                <div class="errormsg">{errorMsg}</div>

                <button type="submit" id="register_button" class="button is-success" onClick={onRegister}>Registrieren</button>
            </form>

            <div>
                Schon registriert? <Link class="button" href="/login">Login</Link>
            </div>
        </div>
    );
}