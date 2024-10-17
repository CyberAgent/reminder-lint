FROM ghcr.io/cyberagent/reminder-lint:latest

COPY entrypoint.sh /usr/local/bin/reminder-lint/
RUN chmod +x /usr/local/bin/reminder-lint/entrypoint.sh

ENTRYPOINT [ "/usr/local/bin/reminder-lint/entrypoint.sh" ]
