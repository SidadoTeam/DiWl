
// const address_contract = "ZfRoeLUReVfGf92nx2GWfTSMEZ9VFAZQxUPQgdAq8T5AvUe";
const address_contract = "5HJBWUZEiQZ8nzzVMm6aDvYT6USL36LewfsFWayPkvkpQBrX";

const contractAbi = {
    "source": {
        "hash": "0xe710d10cfdc5f9de28f304f274bf8b3afc9f755bb0238270c07b573a5958474e",
        "language": "ink! 3.4.0",
        "compiler": "rustc 1.69.0-nightly"
    },
    "contract": {
        "name": "diwl_contract",
        "version": "0.1.0",
        "authors": [
            "zhaojie"
        ]
    },
    "V3": {
        "spec": {
            "constructors": [
                {
                    "args": [],
                    "docs": [],
                    "label": "default",
                    "payable": false,
                    "selector": "0xed4b9d1b"
                }
            ],
            "docs": [],
            "events": [],
            "messages": [
                {
                    "args": [
                        {
                            "label": "track_index",
                            "type": {
                                "displayName": [
                                    "i32"
                                ],
                                "type": 3
                            }
                        },
                        {
                            "label": "track_limit",
                            "type": {
                                "displayName": [
                                    "i32"
                                ],
                                "type": 3
                            }
                        }
                    ],
                    "docs": [],
                    "label": "getw_user",
                    "mutates": false,
                    "payable": false,
                    "returnType": {
                        "displayName": [
                            "Vec"
                        ],
                        "type": 14
                    },
                    "selector": "0x4c0444ff"
                },
                {
                    "args": [
                        {
                            "label": "other_id",
                            "type": {
                                "displayName": [
                                    "AccountId"
                                ],
                                "type": 9
                            }
                        },
                        {
                            "label": "track_index",
                            "type": {
                                "displayName": [
                                    "i32"
                                ],
                                "type": 3
                            }
                        },
                        {
                            "label": "track_limit",
                            "type": {
                                "displayName": [
                                    "i32"
                                ],
                                "type": 3
                            }
                        }
                    ],
                    "docs": [],
                    "label": "getw_other_user",
                    "mutates": false,
                    "payable": false,
                    "returnType": {
                        "displayName": [
                            "Vec"
                        ],
                        "type": 14
                    },
                    "selector": "0xaf6048c4"
                },
                {
                    "args": [
                        {
                            "label": "track_index",
                            "type": {
                                "displayName": [
                                    "i32"
                                ],
                                "type": 3
                            }
                        },
                        {
                            "label": "track_limit",
                            "type": {
                                "displayName": [
                                    "i32"
                                ],
                                "type": 3
                            }
                        }
                    ],
                    "docs": [],
                    "label": "getw_common",
                    "mutates": false,
                    "payable": false,
                    "returnType": {
                        "displayName": [
                            "Vec"
                        ],
                        "type": 14
                    },
                    "selector": "0x4724a11a"
                },
                {
                    "args": [
                        {
                            "label": "word",
                            "type": {
                                "displayName": [
                                    "WordRecord"
                                ],
                                "type": 2
                            }
                        }
                    ],
                    "docs": [],
                    "label": "init_wlist",
                    "mutates": true,
                    "payable": false,
                    "returnType": null,
                    "selector": "0xdb54ed03"
                },
                {
                    "args": [
                        {
                            "label": "word",
                            "type": {
                                "displayName": [
                                    "WordRecord"
                                ],
                                "type": 2
                            }
                        }
                    ],
                    "docs": [],
                    "label": "user_word_in",
                    "mutates": true,
                    "payable": false,
                    "returnType": null,
                    "selector": "0x11591356"
                },
                {
                    "args": [
                        {
                            "label": "taget",
                            "type": {
                                "displayName": [
                                    "AccountId"
                                ],
                                "type": 9
                            }
                        }
                    ],
                    "docs": [],
                    "label": "auth_account",
                    "mutates": false,
                    "payable": false,
                    "returnType": null,
                    "selector": "0x287e57c7"
                }
            ]
        },
        "storage": {
            "struct": {
                "fields": [
                    {
                        "layout": {
                            "cell": {
                                "key": "0x0000000000000000000000000000000000000000000000000000000000000000",
                                "ty": 0
                            }
                        },
                        "name": "g_map"
                    },
                    {
                        "layout": {
                            "cell": {
                                "key": "0x0100000000000000000000000000000000000000000000000000000000000000",
                                "ty": 8
                            }
                        },
                        "name": "user_info"
                    },
                    {
                        "layout": {
                            "cell": {
                                "key": "0x0200000000000000000000000000000000000000000000000000000000000000",
                                "ty": 12
                            }
                        },
                        "name": "user_wlist"
                    },
                    {
                        "layout": {
                            "cell": {
                                "key": "0x0300000000000000000000000000000000000000000000000000000000000000",
                                "ty": 3
                            }
                        },
                        "name": "c_count"
                    },
                    {
                        "layout": {
                            "cell": {
                                "key": "0x0400000000000000000000000000000000000000000000000000000000000000",
                                "ty": 9
                            }
                        },
                        "name": "contract_owner"
                    }
                ]
            }
        },
        "types": [
            {
                "id": 0,
                "type": {
                    "def": {
                        "composite": {
                            "fields": [
                                {
                                    "name": "offset_key",
                                    "type": 5,
                                    "typeName": "Key"
                                }
                            ]
                        }
                    },
                    "params": [
                        {
                            "name": "K",
                            "type": 1
                        },
                        {
                            "name": "V",
                            "type": 2
                        }
                    ],
                    "path": [
                        "ink_storage",
                        "lazy",
                        "mapping",
                        "Mapping"
                    ]
                }
            },
            {
                "id": 1,
                "type": {
                    "def": {
                        "primitive": "str"
                    }
                }
            },
            {
                "id": 2,
                "type": {
                    "def": {
                        "composite": {
                            "fields": [
                                {
                                    "name": "word",
                                    "type": 1,
                                    "typeName": "String"
                                },
                                {
                                    "name": "level",
                                    "type": 3,
                                    "typeName": "i32"
                                },
                                {
                                    "name": "mean",
                                    "type": 1,
                                    "typeName": "String"
                                },
                                {
                                    "name": "hit_count",
                                    "type": 3,
                                    "typeName": "i32"
                                },
                                {
                                    "name": "tag",
                                    "type": 1,
                                    "typeName": "String"
                                },
                                {
                                    "name": "nfts",
                                    "type": 4,
                                    "typeName": "Vec<String>"
                                }
                            ]
                        }
                    },
                    "path": [
                        "diwl_contract",
                        "diwl_contract",
                        "WordRecord"
                    ]
                }
            },
            {
                "id": 3,
                "type": {
                    "def": {
                        "primitive": "i32"
                    }
                }
            },
            {
                "id": 4,
                "type": {
                    "def": {
                        "sequence": {
                            "type": 1
                        }
                    }
                }
            },
            {
                "id": 5,
                "type": {
                    "def": {
                        "composite": {
                            "fields": [
                                {
                                    "type": 6,
                                    "typeName": "[u8; 32]"
                                }
                            ]
                        }
                    },
                    "path": [
                        "ink_primitives",
                        "Key"
                    ]
                }
            },
            {
                "id": 6,
                "type": {
                    "def": {
                        "array": {
                            "len": 32,
                            "type": 7
                        }
                    }
                }
            },
            {
                "id": 7,
                "type": {
                    "def": {
                        "primitive": "u8"
                    }
                }
            },
            {
                "id": 8,
                "type": {
                    "def": {
                        "composite": {
                            "fields": [
                                {
                                    "name": "offset_key",
                                    "type": 5,
                                    "typeName": "Key"
                                }
                            ]
                        }
                    },
                    "params": [
                        {
                            "name": "K",
                            "type": 9
                        },
                        {
                            "name": "V",
                            "type": 10
                        }
                    ],
                    "path": [
                        "ink_storage",
                        "lazy",
                        "mapping",
                        "Mapping"
                    ]
                }
            },
            {
                "id": 9,
                "type": {
                    "def": {
                        "composite": {
                            "fields": [
                                {
                                    "type": 6,
                                    "typeName": "[u8; 32]"
                                }
                            ]
                        }
                    },
                    "path": [
                        "ink_env",
                        "types",
                        "AccountId"
                    ]
                }
            },
            {
                "id": 10,
                "type": {
                    "def": {
                        "composite": {
                            "fields": [
                                {
                                    "name": "c_count",
                                    "type": 3,
                                    "typeName": "i32"
                                },
                                {
                                    "name": "auth_account",
                                    "type": 11,
                                    "typeName": "Vec<AccountId>"
                                }
                            ]
                        }
                    },
                    "path": [
                        "diwl_contract",
                        "diwl_contract",
                        "UserInfo"
                    ]
                }
            },
            {
                "id": 11,
                "type": {
                    "def": {
                        "sequence": {
                            "type": 9
                        }
                    }
                }
            },
            {
                "id": 12,
                "type": {
                    "def": {
                        "composite": {
                            "fields": [
                                {
                                    "name": "offset_key",
                                    "type": 5,
                                    "typeName": "Key"
                                }
                            ]
                        }
                    },
                    "params": [
                        {
                            "name": "K",
                            "type": 13
                        },
                        {
                            "name": "V",
                            "type": 2
                        }
                    ],
                    "path": [
                        "ink_storage",
                        "lazy",
                        "mapping",
                        "Mapping"
                    ]
                }
            },
            {
                "id": 13,
                "type": {
                    "def": {
                        "tuple": [
                            9,
                            3
                        ]
                    }
                }
            },
            {
                "id": 14,
                "type": {
                    "def": {
                        "sequence": {
                            "type": 2
                        }
                    }
                }
            }
        ]
    }
};

export { contractAbi, address_contract }