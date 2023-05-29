// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.30.0
// 	protoc        v3.21.12
// source: proto/words.proto

package words

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type GetWordsRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *GetWordsRequest) Reset() {
	*x = GetWordsRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_proto_words_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetWordsRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetWordsRequest) ProtoMessage() {}

func (x *GetWordsRequest) ProtoReflect() protoreflect.Message {
	mi := &file_proto_words_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetWordsRequest.ProtoReflect.Descriptor instead.
func (*GetWordsRequest) Descriptor() ([]byte, []int) {
	return file_proto_words_proto_rawDescGZIP(), []int{0}
}

type GetWordsResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Timestamp uint64 `protobuf:"varint,1,opt,name=timestamp,proto3" json:"timestamp,omitempty"`
	Word      string `protobuf:"bytes,2,opt,name=word,proto3" json:"word,omitempty"`
}

func (x *GetWordsResponse) Reset() {
	*x = GetWordsResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_proto_words_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetWordsResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetWordsResponse) ProtoMessage() {}

func (x *GetWordsResponse) ProtoReflect() protoreflect.Message {
	mi := &file_proto_words_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetWordsResponse.ProtoReflect.Descriptor instead.
func (*GetWordsResponse) Descriptor() ([]byte, []int) {
	return file_proto_words_proto_rawDescGZIP(), []int{1}
}

func (x *GetWordsResponse) GetTimestamp() uint64 {
	if x != nil {
		return x.Timestamp
	}
	return 0
}

func (x *GetWordsResponse) GetWord() string {
	if x != nil {
		return x.Word
	}
	return ""
}

var File_proto_words_proto protoreflect.FileDescriptor

var file_proto_words_proto_rawDesc = []byte{
	0x0a, 0x11, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x77, 0x6f, 0x72, 0x64, 0x73, 0x2e, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x12, 0x09, 0x77, 0x6f, 0x72, 0x64, 0x73, 0x5f, 0x72, 0x70, 0x63, 0x22, 0x11,
	0x0a, 0x0f, 0x47, 0x65, 0x74, 0x57, 0x6f, 0x72, 0x64, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
	0x74, 0x22, 0x44, 0x0a, 0x10, 0x47, 0x65, 0x74, 0x57, 0x6f, 0x72, 0x64, 0x73, 0x52, 0x65, 0x73,
	0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61,
	0x6d, 0x70, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74,
	0x61, 0x6d, 0x70, 0x12, 0x12, 0x0a, 0x04, 0x77, 0x6f, 0x72, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28,
	0x09, 0x52, 0x04, 0x77, 0x6f, 0x72, 0x64, 0x32, 0x50, 0x0a, 0x05, 0x57, 0x6f, 0x72, 0x64, 0x73,
	0x12, 0x47, 0x0a, 0x08, 0x47, 0x65, 0x74, 0x57, 0x6f, 0x72, 0x64, 0x73, 0x12, 0x1a, 0x2e, 0x77,
	0x6f, 0x72, 0x64, 0x73, 0x5f, 0x72, 0x70, 0x63, 0x2e, 0x47, 0x65, 0x74, 0x57, 0x6f, 0x72, 0x64,
	0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x1b, 0x2e, 0x77, 0x6f, 0x72, 0x64, 0x73,
	0x5f, 0x72, 0x70, 0x63, 0x2e, 0x47, 0x65, 0x74, 0x57, 0x6f, 0x72, 0x64, 0x73, 0x52, 0x65, 0x73,
	0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x30, 0x01, 0x42, 0x15, 0x5a, 0x13, 0x67, 0x6f, 0x5f,
	0x72, 0x70, 0x63, 0x5f, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2f, 0x77, 0x6f, 0x72, 0x64, 0x73,
	0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_proto_words_proto_rawDescOnce sync.Once
	file_proto_words_proto_rawDescData = file_proto_words_proto_rawDesc
)

func file_proto_words_proto_rawDescGZIP() []byte {
	file_proto_words_proto_rawDescOnce.Do(func() {
		file_proto_words_proto_rawDescData = protoimpl.X.CompressGZIP(file_proto_words_proto_rawDescData)
	})
	return file_proto_words_proto_rawDescData
}

var file_proto_words_proto_msgTypes = make([]protoimpl.MessageInfo, 2)
var file_proto_words_proto_goTypes = []interface{}{
	(*GetWordsRequest)(nil),  // 0: words_rpc.GetWordsRequest
	(*GetWordsResponse)(nil), // 1: words_rpc.GetWordsResponse
}
var file_proto_words_proto_depIdxs = []int32{
	0, // 0: words_rpc.Words.GetWords:input_type -> words_rpc.GetWordsRequest
	1, // 1: words_rpc.Words.GetWords:output_type -> words_rpc.GetWordsResponse
	1, // [1:2] is the sub-list for method output_type
	0, // [0:1] is the sub-list for method input_type
	0, // [0:0] is the sub-list for extension type_name
	0, // [0:0] is the sub-list for extension extendee
	0, // [0:0] is the sub-list for field type_name
}

func init() { file_proto_words_proto_init() }
func file_proto_words_proto_init() {
	if File_proto_words_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_proto_words_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetWordsRequest); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_proto_words_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetWordsResponse); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_proto_words_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   2,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_proto_words_proto_goTypes,
		DependencyIndexes: file_proto_words_proto_depIdxs,
		MessageInfos:      file_proto_words_proto_msgTypes,
	}.Build()
	File_proto_words_proto = out.File
	file_proto_words_proto_rawDesc = nil
	file_proto_words_proto_goTypes = nil
	file_proto_words_proto_depIdxs = nil
}
