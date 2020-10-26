# srcrr
# Copyright (c) 2020, Colvin Wellborn
# vim: ft=sh

function src() {
	: ${SRCRR_BIN:="srcrr"}
	case "$1" in
		-l|--list)
			shift
			$SRCRR_BIN -l $@
			return;;
		-d|--dirs)
			shift
			$SRCRR_BIN -d $@
			return;;
		-V|--version)
			$SRCRR_BIN -V
			return;;
		-h|--help)
			$SRCRR_BIN -h
			return;;
	esac
	eval $( $SRCRR_BIN $@ )
}

# Bash completion support for srcrr.
function _src_comp() {
	: ${SRCRR_BIN:="srcrr"}
	unset _src_words
	for proj in $( $SRCRR_BIN -l | sort -u )
	do
		_src_words+="$proj "
	done
	COMPREPLY+=( $( compgen -W "$_src_words" "${COMP_WORDS[1]}" ) )
}

complete -F _src_comp src
