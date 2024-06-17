import rpg_api
from rpg_api import App
from openai import OpenAI
from typing import List
import time

global conf
global app
global openai
global request_threads

def main():
    global conf
    global app
    global openai
    global request_threads

    request_threads = []

    conf = rpg_api.load_config("config.toml")
    app = App(conf)

    app.run()

    openai = OpenAI(conf.get_openai_api_key())

    while True:
        cont = True
        request = app.check_requests()
        if request:
            response = handle_request(request[0], request[1])
            app.handle_request(response)
            cont = False

        new_thread_request = app.check_new_thread_requests()
        if new_thread_request:
            response = new_request_thread(new_thread_request)
            app.new_thread(response)
            cont = False

        if cont: time.sleep(0.02)

def new_request_thread(system_role: str) -> int:
    global openai
    global request_threads

    request_threads.append(
        openai.chat.completions.create(
            model="gpt-3.5-turbo",
            messages=[ { "role": "system", "content": system_role } ]
        )
    )

    return len(request_threads) - 1

def handle_request(thread_id: int, request: str) -> str:
    global app
    global request_threads

    request_threads[thread_id].messages.append(
        { "role": "user", "content": request }
    )

    response = request_threads[thread_id].execute()

    return response.choices[0].message.content

if __name__ == "__main__":
    main()