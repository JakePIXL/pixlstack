import type { Actions } from './$types';
import { TURNSTILE_TOKEN } from '$env/static/private';
 
export const actions = {
  default: async (event) => {

    const data = await event.request.formData();

    const user_turnstile_token = data.get("cf-turnstile-response");
    const email = data.get("email");
    const name = data.get("name");
    const company_name = data.get("company");
    const message = data.get("message");
    const urgency = data.get("urgency");

    let ip = event.request.headers.get("CF-Connecting-IP");
    if (!ip) {
      ip = "0.0.0.0";
    }
    
    let turnstile_completed = false;

    await fetch("https://challenges.cloudflare.com/turnstile/v0/siteverify", {
      method: "POST",
      headers: {"Content-Type": "application/json"},
      body: JSON.stringify({
        secret: TURNSTILE_TOKEN,
        response: user_turnstile_token,
        remoteip: ip
      })
    })
      .then(async response => {
        let response_data = await response.json();

        if (response_data.sucess === "true") {
          turnstile_completed = true;
        }
      })
      .catch(error => {
        return { sucess: false, error: { message: error }}
      });

    if (turnstile_completed) {
      await fetch("/api/contact", {
        method: "POST",
        headers: {"Content-Type": "application/json"},
        body: JSON.stringify({
          name,
          email,
          company_name,
          message,
          urgency
        })
      })
      .then(async response => {
        if (await response.status === 201) {
          return { success: true }
        } else {
          return { success: false, error: await response.json() }
        }
      })
      .catch(error => {
        return { sucess: false, error: { message: "Error occured while submitting contact request, try again later." }}
      })
    }    
  }
} satisfies Actions;