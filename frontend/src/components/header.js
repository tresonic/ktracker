import '../header'

import { Link, route } from "preact-router";
import authService from "../services/auth.service";
import { useCallback, useEffect, useState } from 'preact/hooks';

export default function Header(props) {
    const [, updateState] = useState();
    const forceUpdate = useCallback(() => updateState({}), []);


    useEffect(() => {
        window.addEventListener("popstate", forceUpdate);
        return () => {
            window.removeEventListener("popstate", forceUpdate);
        };
    }, []);

    const onLogout = e => {
        authService.logout();
        forceUpdate();
        route("/login");
    }

    return (
        <header>
            <nav role="navigation" aria-label="main navigation">
                <div class="brand"><Link class="brandcombo" href="/overview"><img class="icon" src="assets/favicon.ico" alt="icon" />&nbsp;kTracker</Link></div>
                <ul>
                    {authService.isLoggedIn() ?
                        <>
                            <li><Link class="navitem" href="/highscore">Highscore</Link></li>
                            <li><div class="navitem" onClick={onLogout}>Abmelden</div></li>
                        </>
                        :
                        <>
                            <li><Link class="navitem" href="/login">Anmelden</Link></li>
                        </>
                    }
                </ul>
            </nav>
        </header>
    );
}