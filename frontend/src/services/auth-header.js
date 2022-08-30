export default function authHeader() {
    let user = null;
    
    console.log("auth-header");

    if(typeof window !== "undefined") {
        // console.log("auth-header load");
        user = JSON.parse(localStorage.getItem('user'));
    }

    if (user && user.access_token) {
        // console.log("with auth")
        return { Authorization: 'Bearer ' + user.access_token };
    } else {
        return {};
    }
}
