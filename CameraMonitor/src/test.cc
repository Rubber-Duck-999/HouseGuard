#include <iostream>
#include "boostlogging.h"

int main()
{
    init_log();
    BOOST_LOG_TRIVIAL(trace) << "this is a trace message";
    BOOST_LOG_TRIVIAL(debug) << "this is a debug message";
    BOOST_LOG_TRIVIAL(info) << "this is a info message";
    BOOST_LOG_TRIVIAL(warning) << "this is a warning message";
    BOOST_LOG_TRIVIAL(error) << "this is an error message";
    BOOST_LOG_TRIVIAL(fatal) << "this is a fatal error message";
    return 0;
}
