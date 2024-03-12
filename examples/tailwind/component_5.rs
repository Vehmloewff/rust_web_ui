use rust_web_ui::*;

pub struct Example5;

pub struct Example5Props {}

impl Default for Example5Props {
	fn default() -> Example5Props {
		Example5Props { }
	}
}

impl Widget<'_> for Example5 {
	type Props = Example5Props;

	fn render(mut ctx: Ctx<'_>, props: Example5Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("bg-white")]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-7xl"), Style::PaddingX(16), Style::PaddingY(96), Screen::Small(&[Style::PaddingX(24)]), Screen::Small(&[Style::PaddingY(128)]), Screen::Large(&[Style::PaddingX(32)])]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-2xl")]);
					
					props.child("1", Dynamic).run("form", |props| {
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Style::SpaceY(48)]);
							
							props.child("1", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("border-b"), NoStyle::Noop("border-gray-900/10"), Style::PaddingBottom(48)]);
								
								props.child("1", Dynamic).run("h2", |props| {
									props.styles(&[NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.set_text("Profile"));
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(67))]);
									
									props.child("0", Label).run(|props| props.set_text("This information will be displayed publicly so be careful what you share."));
								});
								props.child("5", Dynamic).run("div", |props| {
									props.styles(&[Style::MarginTop(40), NoStyle::Noop("grid"), NoStyle::Noop("grid-cols-1"), NoStyle::Noop("gap-x-6"), NoStyle::Noop("gap-y-8"), Screen::Small(&[NoStyle::Noop("grid-cols-6")])]);
									
									props.child("1", Dynamic).run("div", |props| {
										props.styles(&[Screen::Small(&[NoStyle::Noop("col-span-4")])]);
										
										props.child("1", Dynamic).run("label", |props| {
											props.set_attribute("for", "username");
											props.styles(&[Style::Block, NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.set_text("Username"));
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[Style::MarginTop(8)]);
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, NoStyle::Noop("rounded-md"), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), NoStyle::NoopGroup("focus-within", NoStyle::Noop("ring-2")), NoStyle::NoopGroup("focus-within", NoStyle::Noop("ring-inset")), NoStyle::NoopGroup("focus-within", NoStyle::Noop("ring-indigo-600")), Screen::Small(&[NoStyle::Noop("max-w-md")])]);
												
												props.child("1", Dynamic).run("span", |props| {
													props.styles(&[Style::Flex, NoStyle::Noop("select-none"), Style::ItemsCenter, Style::PaddingLeft(12), Style::TextColor(Color::Fg(56)), Screen::Small(&[NoStyle::Noop("text-sm")])]);
													
													props.child("0", Label).run(|props| props.set_text("workcation.com/"));
												});
												props.child("3", Dynamic).run("input", |props| {
													props.set_attribute("type", "text");
													props.set_attribute("name", "username");
													props.set_attribute("id", "username");
													props.set_attribute("autocomplete", "username");
													props.styles(&[Style::Block, NoStyle::Noop("flex-1"), NoStyle::Noop("border-0"), NoStyle::Noop("bg-transparent"), Style::PaddingY(6), Style::PaddingLeft(4), Style::TextColor(Color::Fg(100)), NoStyle::NoopGroup("placeholder", Style::TextColor(Color::Fg(44))), Action::Hover(&[NoStyle::Noop("ring-0")]), Screen::Small(&[NoStyle::Noop("text-sm")]), Screen::Small(&[NoStyle::Noop("leading-6")])]);
													props.set_attribute("placeholder", "janesmith");
												});
											});
										});
									});
									props.child("3", Dynamic).run("div", |props| {
										props.styles(&[NoStyle::Noop("col-span-full")]);
										
										props.child("1", Dynamic).run("label", |props| {
											props.set_attribute("for", "about");
											props.styles(&[Style::Block, NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.set_text("About"));
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[Style::MarginTop(8)]);
											
											props.child("1", Dynamic).run("textarea", |props| {
												props.set_attribute("id", "about");
												props.set_attribute("name", "about");
												props.set_attribute("rows", "3");
												props.styles(&[Style::Block, NoStyle::Noop("w-full"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border-0"), Style::PaddingY(6), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), NoStyle::NoopGroup("placeholder", Style::TextColor(Color::Fg(44))), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-inset")]), Action::Hover(&[NoStyle::Noop("ring-indigo-600")]), Screen::Small(&[NoStyle::Noop("text-sm")]), Screen::Small(&[NoStyle::Noop("leading-6")])]);
											});
										});
										props.child("5", Dynamic).run("p", |props| {
											props.styles(&[Style::MarginTop(12), NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(67))]);
											
											props.child("0", Label).run(|props| props.set_text("Write a few sentences about yourself."));
										});
									});
									props.child("5", Dynamic).run("div", |props| {
										props.styles(&[NoStyle::Noop("col-span-full")]);
										
										props.child("1", Dynamic).run("label", |props| {
											props.set_attribute("for", "photo");
											props.styles(&[Style::Block, NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.set_text("Photo"));
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[Style::MarginTop(8), Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-3")]);
											
											props.child("1", Icon).run(|props| {
												props.style(&[Style::Width(48), Style::Width(48), Style::TextColor(Color::Fg(33))]);
											});
											props.child("3", Dynamic).run("button", |props| {
												props.set_attribute("type", "button");
												props.styles(&[NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-white"), Style::PaddingX(10), Style::PaddingY(6), NoStyle::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), Action::Hover(&[Style::Color(Color::Fg(6))])]);
												
												props.child("0", Label).run(|props| props.set_text("Change"));
											});
										});
									});
									props.child("7", Dynamic).run("div", |props| {
										props.styles(&[NoStyle::Noop("col-span-full")]);
										
										props.child("1", Dynamic).run("label", |props| {
											props.set_attribute("for", "cover-photo");
											props.styles(&[Style::Block, NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.set_text("Cover photo"));
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[Style::MarginTop(8), Style::Flex, Style::JustifyCenter, NoStyle::Noop("rounded-lg"), NoStyle::Noop("border"), NoStyle::Noop("border-dashed"), NoStyle::Noop("border-gray-900/25"), Style::PaddingX(24), Style::PaddingY(40)]);
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[NoStyle::Noop("text-center")]);
												
												props.child("1", Icon).run(|props| {
													props.style(&[NoStyle::Noop("mx-auto"), Style::Width(48), Style::Width(48), Style::TextColor(Color::Fg(33))]);
												});
												props.child("3", Dynamic).run("div", |props| {
													props.styles(&[Style::MarginTop(16), Style::Flex, NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(67))]);
													
													props.child("1", Dynamic).run("label", |props| {
														props.set_attribute("for", "file-upload");
														props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("cursor-pointer"), NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-white"), Style::FontSemibold, NoStyle::Noop("text-indigo-600"), NoStyle::NoopGroup("focus-within", NoStyle::Noop("outline-none")), NoStyle::NoopGroup("focus-within", NoStyle::Noop("ring-2")), NoStyle::NoopGroup("focus-within", NoStyle::Noop("ring-indigo-600")), NoStyle::NoopGroup("focus-within", NoStyle::Noop("ring-offset-2")), Action::Hover(&[NoStyle::Noop("text-indigo-500")])]);
														
														props.child("1", Dynamic).run("span", |props| {
															props.child("0", Label).run(|props| props.set_text("Upload a file"));
														});
														props.child("3", Dynamic).run("input", |props| {
															props.set_attribute("id", "file-upload");
															props.set_attribute("name", "file-upload");
															props.set_attribute("type", "file");
															props.styles(&[NoStyle::Noop("sr-only")]);
														});
													});
													props.child("3", Dynamic).run("p", |props| {
														props.styles(&[Style::PaddingLeft(4)]);
														
														props.child("0", Label).run(|props| props.set_text("or drag and drop"));
													});
												});
												props.child("5", Dynamic).run("p", |props| {
													props.styles(&[NoStyle::Noop("text-xs"), NoStyle::Noop("leading-5"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.set_text("PNG, JPG, GIF up to 10MB"));
												});
											});
										});
									});
								});
							});
							props.child("3", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("border-b"), NoStyle::Noop("border-gray-900/10"), Style::PaddingBottom(48)]);
								
								props.child("1", Dynamic).run("h2", |props| {
									props.styles(&[NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.set_text("Personal Information"));
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(67))]);
									
									props.child("0", Label).run(|props| props.set_text("Use a permanent address where you can receive mail."));
								});
								props.child("5", Dynamic).run("div", |props| {
									props.styles(&[Style::MarginTop(40), NoStyle::Noop("grid"), NoStyle::Noop("grid-cols-1"), NoStyle::Noop("gap-x-6"), NoStyle::Noop("gap-y-8"), Screen::Small(&[NoStyle::Noop("grid-cols-6")])]);
									
									props.child("1", Dynamic).run("div", |props| {
										props.styles(&[Screen::Small(&[NoStyle::Noop("col-span-3")])]);
										
										props.child("1", Dynamic).run("label", |props| {
											props.set_attribute("for", "first-name");
											props.styles(&[Style::Block, NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.set_text("First name"));
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[Style::MarginTop(8)]);
											
											props.child("1", Dynamic).run("input", |props| {
												props.set_attribute("type", "text");
												props.set_attribute("name", "first-name");
												props.set_attribute("id", "first-name");
												props.set_attribute("autocomplete", "given-name");
												props.styles(&[Style::Block, NoStyle::Noop("w-full"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border-0"), Style::PaddingY(6), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), NoStyle::NoopGroup("placeholder", Style::TextColor(Color::Fg(44))), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-inset")]), Action::Hover(&[NoStyle::Noop("ring-indigo-600")]), Screen::Small(&[NoStyle::Noop("text-sm")]), Screen::Small(&[NoStyle::Noop("leading-6")])]);
											});
										});
									});
									props.child("3", Dynamic).run("div", |props| {
										props.styles(&[Screen::Small(&[NoStyle::Noop("col-span-3")])]);
										
										props.child("1", Dynamic).run("label", |props| {
											props.set_attribute("for", "last-name");
											props.styles(&[Style::Block, NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.set_text("Last name"));
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[Style::MarginTop(8)]);
											
											props.child("1", Dynamic).run("input", |props| {
												props.set_attribute("type", "text");
												props.set_attribute("name", "last-name");
												props.set_attribute("id", "last-name");
												props.set_attribute("autocomplete", "family-name");
												props.styles(&[Style::Block, NoStyle::Noop("w-full"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border-0"), Style::PaddingY(6), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), NoStyle::NoopGroup("placeholder", Style::TextColor(Color::Fg(44))), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-inset")]), Action::Hover(&[NoStyle::Noop("ring-indigo-600")]), Screen::Small(&[NoStyle::Noop("text-sm")]), Screen::Small(&[NoStyle::Noop("leading-6")])]);
											});
										});
									});
									props.child("5", Dynamic).run("div", |props| {
										props.styles(&[Screen::Small(&[NoStyle::Noop("col-span-4")])]);
										
										props.child("1", Dynamic).run("label", |props| {
											props.set_attribute("for", "email");
											props.styles(&[Style::Block, NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.set_text("Email address"));
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[Style::MarginTop(8)]);
											
											props.child("1", Dynamic).run("input", |props| {
												props.set_attribute("id", "email");
												props.set_attribute("name", "email");
												props.set_attribute("type", "email");
												props.set_attribute("autocomplete", "email");
												props.styles(&[Style::Block, NoStyle::Noop("w-full"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border-0"), Style::PaddingY(6), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), NoStyle::NoopGroup("placeholder", Style::TextColor(Color::Fg(44))), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-inset")]), Action::Hover(&[NoStyle::Noop("ring-indigo-600")]), Screen::Small(&[NoStyle::Noop("text-sm")]), Screen::Small(&[NoStyle::Noop("leading-6")])]);
											});
										});
									});
									props.child("7", Dynamic).run("div", |props| {
										props.styles(&[Screen::Small(&[NoStyle::Noop("col-span-3")])]);
										
										props.child("1", Dynamic).run("label", |props| {
											props.set_attribute("for", "country");
											props.styles(&[Style::Block, NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.set_text("Country"));
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[Style::MarginTop(8)]);
											
											props.child("1", Dynamic).run("select", |props| {
												props.set_attribute("id", "country");
												props.set_attribute("name", "country");
												props.set_attribute("autocomplete", "country-name");
												props.styles(&[Style::Block, NoStyle::Noop("w-full"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border-0"), Style::PaddingY(6), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-inset")]), Action::Hover(&[NoStyle::Noop("ring-indigo-600")]), Screen::Small(&[NoStyle::Noop("max-w-xs")]), Screen::Small(&[NoStyle::Noop("text-sm")]), Screen::Small(&[NoStyle::Noop("leading-6")])]);
												
												props.child("1", Dynamic).run("option", |props| {
													props.child("0", Label).run(|props| props.set_text("United States"));
												});
												props.child("3", Dynamic).run("option", |props| {
													props.child("0", Label).run(|props| props.set_text("Canada"));
												});
												props.child("5", Dynamic).run("option", |props| {
													props.child("0", Label).run(|props| props.set_text("Mexico"));
												});
											});
										});
									});
									props.child("9", Dynamic).run("div", |props| {
										props.styles(&[NoStyle::Noop("col-span-full")]);
										
										props.child("1", Dynamic).run("label", |props| {
											props.set_attribute("for", "street-address");
											props.styles(&[Style::Block, NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.set_text("Street address"));
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[Style::MarginTop(8)]);
											
											props.child("1", Dynamic).run("input", |props| {
												props.set_attribute("type", "text");
												props.set_attribute("name", "street-address");
												props.set_attribute("id", "street-address");
												props.set_attribute("autocomplete", "street-address");
												props.styles(&[Style::Block, NoStyle::Noop("w-full"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border-0"), Style::PaddingY(6), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), NoStyle::NoopGroup("placeholder", Style::TextColor(Color::Fg(44))), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-inset")]), Action::Hover(&[NoStyle::Noop("ring-indigo-600")]), Screen::Small(&[NoStyle::Noop("text-sm")]), Screen::Small(&[NoStyle::Noop("leading-6")])]);
											});
										});
									});
									props.child("11", Dynamic).run("div", |props| {
										props.styles(&[Screen::Small(&[NoStyle::Noop("col-span-2")]), Screen::Small(&[NoStyle::Noop("col-start-1")])]);
										
										props.child("1", Dynamic).run("label", |props| {
											props.set_attribute("for", "city");
											props.styles(&[Style::Block, NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.set_text("City"));
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[Style::MarginTop(8)]);
											
											props.child("1", Dynamic).run("input", |props| {
												props.set_attribute("type", "text");
												props.set_attribute("name", "city");
												props.set_attribute("id", "city");
												props.set_attribute("autocomplete", "address-level2");
												props.styles(&[Style::Block, NoStyle::Noop("w-full"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border-0"), Style::PaddingY(6), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), NoStyle::NoopGroup("placeholder", Style::TextColor(Color::Fg(44))), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-inset")]), Action::Hover(&[NoStyle::Noop("ring-indigo-600")]), Screen::Small(&[NoStyle::Noop("text-sm")]), Screen::Small(&[NoStyle::Noop("leading-6")])]);
											});
										});
									});
									props.child("13", Dynamic).run("div", |props| {
										props.styles(&[Screen::Small(&[NoStyle::Noop("col-span-2")])]);
										
										props.child("1", Dynamic).run("label", |props| {
											props.set_attribute("for", "region");
											props.styles(&[Style::Block, NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.set_text("State / Province"));
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[Style::MarginTop(8)]);
											
											props.child("1", Dynamic).run("input", |props| {
												props.set_attribute("type", "text");
												props.set_attribute("name", "region");
												props.set_attribute("id", "region");
												props.set_attribute("autocomplete", "address-level1");
												props.styles(&[Style::Block, NoStyle::Noop("w-full"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border-0"), Style::PaddingY(6), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), NoStyle::NoopGroup("placeholder", Style::TextColor(Color::Fg(44))), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-inset")]), Action::Hover(&[NoStyle::Noop("ring-indigo-600")]), Screen::Small(&[NoStyle::Noop("text-sm")]), Screen::Small(&[NoStyle::Noop("leading-6")])]);
											});
										});
									});
									props.child("15", Dynamic).run("div", |props| {
										props.styles(&[Screen::Small(&[NoStyle::Noop("col-span-2")])]);
										
										props.child("1", Dynamic).run("label", |props| {
											props.set_attribute("for", "postal-code");
											props.styles(&[Style::Block, NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.set_text("ZIP / Postal code"));
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[Style::MarginTop(8)]);
											
											props.child("1", Dynamic).run("input", |props| {
												props.set_attribute("type", "text");
												props.set_attribute("name", "postal-code");
												props.set_attribute("id", "postal-code");
												props.set_attribute("autocomplete", "postal-code");
												props.styles(&[Style::Block, NoStyle::Noop("w-full"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border-0"), Style::PaddingY(6), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), NoStyle::NoopGroup("placeholder", Style::TextColor(Color::Fg(44))), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-inset")]), Action::Hover(&[NoStyle::Noop("ring-indigo-600")]), Screen::Small(&[NoStyle::Noop("text-sm")]), Screen::Small(&[NoStyle::Noop("leading-6")])]);
											});
										});
									});
								});
							});
							props.child("5", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("border-b"), NoStyle::Noop("border-gray-900/10"), Style::PaddingBottom(48)]);
								
								props.child("1", Dynamic).run("h2", |props| {
									props.styles(&[NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.set_text("Notifications"));
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(67))]);
									
									props.child("0", Label).run(|props| props.set_text("We'll always let you know about important changes, but you pick what else you want to hear about."));
								});
								props.child("5", Dynamic).run("div", |props| {
									props.styles(&[Style::MarginTop(40), Style::SpaceY(40)]);
									
									props.child("1", Dynamic).run("fieldset", |props| {
										props.child("1", Dynamic).run("legend", |props| {
											props.styles(&[NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.set_text("By Email"));
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[Style::MarginTop(24), Style::SpaceY(24)]);
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[NoStyle::Noop("relative"), Style::Flex, NoStyle::Noop("gap-x-3")]);
												
												props.child("1", Dynamic).run("div", |props| {
													props.styles(&[Style::Flex, Style::Width(24), Style::ItemsCenter]);
													
													props.child("1", Dynamic).run("input", |props| {
														props.set_attribute("id", "comments");
														props.set_attribute("name", "comments");
														props.set_attribute("type", "checkbox");
														props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-600")])]);
													});
												});
												props.child("3", Dynamic).run("div", |props| {
													props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6")]);
													
													props.child("1", Dynamic).run("label", |props| {
														props.set_attribute("for", "comments");
														props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
														
														props.child("0", Label).run(|props| props.set_text("Comments"));
													});
													props.child("3", Dynamic).run("p", |props| {
														props.styles(&[Style::TextColor(Color::Fg(56))]);
														
														props.child("0", Label).run(|props| props.set_text("Get notified when someones posts a comment on a posting."));
													});
												});
											});
											props.child("3", Dynamic).run("div", |props| {
												props.styles(&[NoStyle::Noop("relative"), Style::Flex, NoStyle::Noop("gap-x-3")]);
												
												props.child("1", Dynamic).run("div", |props| {
													props.styles(&[Style::Flex, Style::Width(24), Style::ItemsCenter]);
													
													props.child("1", Dynamic).run("input", |props| {
														props.set_attribute("id", "candidates");
														props.set_attribute("name", "candidates");
														props.set_attribute("type", "checkbox");
														props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-600")])]);
													});
												});
												props.child("3", Dynamic).run("div", |props| {
													props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6")]);
													
													props.child("1", Dynamic).run("label", |props| {
														props.set_attribute("for", "candidates");
														props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
														
														props.child("0", Label).run(|props| props.set_text("Candidates"));
													});
													props.child("3", Dynamic).run("p", |props| {
														props.styles(&[Style::TextColor(Color::Fg(56))]);
														
														props.child("0", Label).run(|props| props.set_text("Get notified when a candidate applies for a job."));
													});
												});
											});
											props.child("5", Dynamic).run("div", |props| {
												props.styles(&[NoStyle::Noop("relative"), Style::Flex, NoStyle::Noop("gap-x-3")]);
												
												props.child("1", Dynamic).run("div", |props| {
													props.styles(&[Style::Flex, Style::Width(24), Style::ItemsCenter]);
													
													props.child("1", Dynamic).run("input", |props| {
														props.set_attribute("id", "offers");
														props.set_attribute("name", "offers");
														props.set_attribute("type", "checkbox");
														props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-600")])]);
													});
												});
												props.child("3", Dynamic).run("div", |props| {
													props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6")]);
													
													props.child("1", Dynamic).run("label", |props| {
														props.set_attribute("for", "offers");
														props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
														
														props.child("0", Label).run(|props| props.set_text("Offers"));
													});
													props.child("3", Dynamic).run("p", |props| {
														props.styles(&[Style::TextColor(Color::Fg(56))]);
														
														props.child("0", Label).run(|props| props.set_text("Get notified when a candidate accepts or rejects an offer."));
													});
												});
											});
										});
									});
									props.child("3", Dynamic).run("fieldset", |props| {
										props.child("1", Dynamic).run("legend", |props| {
											props.styles(&[NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.set_text("Push Notifications"));
										});
										props.child("3", Dynamic).run("p", |props| {
											props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(67))]);
											
											props.child("0", Label).run(|props| props.set_text("These are delivered via SMS to your mobile phone."));
										});
										props.child("5", Dynamic).run("div", |props| {
											props.styles(&[Style::MarginTop(24), Style::SpaceY(24)]);
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-3")]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "push-everything");
													props.set_attribute("name", "push-notifications");
													props.set_attribute("type", "radio");
													props.styles(&[Style::Width(16), Style::Width(16), NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-600")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "push-everything");
													props.styles(&[Style::Block, NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
													
													props.child("0", Label).run(|props| props.set_text("Everything"));
												});
											});
											props.child("3", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-3")]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "push-email");
													props.set_attribute("name", "push-notifications");
													props.set_attribute("type", "radio");
													props.styles(&[Style::Width(16), Style::Width(16), NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-600")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "push-email");
													props.styles(&[Style::Block, NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
													
													props.child("0", Label).run(|props| props.set_text("Same as email"));
												});
											});
											props.child("5", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-3")]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "push-nothing");
													props.set_attribute("name", "push-notifications");
													props.set_attribute("type", "radio");
													props.styles(&[Style::Width(16), Style::Width(16), NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-600")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "push-nothing");
													props.styles(&[Style::Block, NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
													
													props.child("0", Label).run(|props| props.set_text("No push notifications"));
												});
											});
										});
									});
								});
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[Style::MarginTop(24), Style::Flex, Style::ItemsCenter, NoStyle::Noop("justify-end"), NoStyle::Noop("gap-x-6")]);
							
							props.child("1", Dynamic).run("button", |props| {
								props.set_attribute("type", "button");
								props.styles(&[NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.set_text("Cancel"));
							});
							props.child("3", Dynamic).run("button", |props| {
								props.set_attribute("type", "submit");
								props.styles(&[NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-indigo-600"), Style::PaddingX(12), Style::PaddingY(8), NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("text-white"), NoStyle::Noop("shadow-sm"), Action::Hover(&[NoStyle::Noop("bg-indigo-500")]), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-2")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-offset-2")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-indigo-600"))]);
								
								props.child("0", Label).run(|props| props.set_text("Save"));
							});
						});
					});
				});
			});
		});
	}
}
