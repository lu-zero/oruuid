from oruuid import uuid1

def test_uuid1():
    u = uuid1()
    assert u.version == 1
