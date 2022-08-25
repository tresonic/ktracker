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
        // <p>Test</p>
        <div class="modal is-active">
            <div class="modal-background"></div>
            <div class="modal-card">
                <header class="modal-card-head">
                    <p class="modal-card-title">Registrieren</p>
                </header>
                <section class="modal-card-body">
                    <div class="field">
                        <label class="label">Benutzername</label>
                        <div class="control">
                            <input class="input is-success" type="text" value={username} onInput={onUsernameInput}/>
                        </div>
                    </div>

                    <div class="field">
                        <label class="label">Email</label>
                        <div class="control">
                            <input class="input is-success" type="email" value={email} onInput={onEmailInput}/>
                        </div>
                    </div>

                    <div class="field">
                    <label class="label">Passwort</label>
                        <p class="control">
                            <input class="input is-dark" type="password" value={pass} onInput={onPassInput}/>
                        </p>
                    </div>
                    <div class="has-text-danger">{errorMsg}</div>
                </section>
                <footer class="modal-card-foot">
                    <button id="register_button" class="button is-success" onClick={onRegister}>Registrieren</button>
                    <Link class="button" href="/login">Login</Link>
                </footer>
            </div>
        </div>
    );
}