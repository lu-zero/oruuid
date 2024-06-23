from oruuid import uuid7

def test_uuid7():
    u = uuid7()
    assert u.version == 7
