import pytest
from intelx import main

# @pytest.mark.skip(reason="Enahcement Entry Point")
def test_main_not_apikey():
    with pytest.raises(SystemExit) as pytest_wrapped_e:
        main(["-search", "apple.com"])
    assert pytest_wrapped_e.value.code == 'No API key specified. Please use the \"-apikey\" parameter or set the environment variable \"INTELX_KEY\".'
