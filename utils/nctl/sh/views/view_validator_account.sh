#!/usr/bin/env bash

source "$NCTL"/sh/utils/main.sh
source "$NCTL"/sh/views/utils.sh

#######################################
# Renders node on-chain account details.
# Globals:
#   NCTL_ACCOUNT_TYPE_NODE - node account type literal.
# Arguments:
#   Node ordinal identifier.
#######################################
function main()
{
    local NODE_ID=${1}

    if [ "$NODE_ID" = "all" ]; then
        for NODE_ID in $(seq 1 "$(get_count_of_nodes)")
        do
            echo "------------------------------------------------------------------------------------------------------------------------------------"
            log "node #$NODE_ID :: on-chain account details:"
            render_account "$NCTL_ACCOUNT_TYPE_NODE" "$NODE_ID"
        done
    else
        log "node #$NODE_ID :: on-chain account details:"
        render_account "$NCTL_ACCOUNT_TYPE_NODE" "$NODE_ID"
    fi
}

# ----------------------------------------------------------------
# ENTRY POINT
# ----------------------------------------------------------------

unset NODE_ID

for ARGUMENT in "$@"
do
    KEY=$(echo "$ARGUMENT" | cut -f1 -d=)
    VALUE=$(echo "$ARGUMENT" | cut -f2 -d=)
    case "$KEY" in
        node) NODE_ID=${VALUE} ;;
        *)
    esac
done

main "${NODE_ID:-"all"}"
