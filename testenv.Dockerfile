FROM debian:stable-slim
RUN apt-get update && apt-get install -y git neovim tmux
COPY target/x86_64-unknown-linux-musl/release/dotrs /usr/local/bin/dotrs
COPY bin/wrapper-bash.sh /usr/local/bin/dotrs-wrapper.sh
WORKDIR /root
RUN dotrs import https://git.zekro.de/zekro/dftest.git
CMD /usr/bin/tmux new-session \;\
    send-keys "source /usr/local/bin/dotrs-wrapper.sh" C-m C-l \;\
    split -h \;\
    send-keys "source /usr/local/bin/dotrs-wrapper.sh" C-m C-l \;\
    bind k kill-session \;\
    select-pane -t 0
