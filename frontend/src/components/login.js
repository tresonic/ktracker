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
            route("/", true);
        }
        // console.log(res);
    }

    return (
        <div class="modal is-active">
            <div class="modal-background"></div>
            <div class="modal-card">
                <header class="modal-card-head">
                    <p class="modal-card-title">Login</p>
                </header>
                <section class="modal-card-body">
                    <div class="field">
                        <label class="label">Benutzername</label>
                        <div class="control">
                            <input class="input is-success" type="text" value={username} onInput={onUsernameInput} />
                        </div>
                    </div>

                    <div class="field">
                        <label class="label">Passwort</label>
                        <p class="control">
                            <input class="input is-dark" type="password" value={pass} onInput={onPassInput} />
                        </p>
                    </div>
                    <div class="has-text-danger">{errorMsg}</div>
                </section>
                <footer class="modal-card-foot">
                    <button id="login_button" class="button is-success" onClick={onLogin} type="submit">Login</button>
                    <Link class="button" href="/register">Registrieren</Link>
                </footer>
            </div>
        </div>
    );
}