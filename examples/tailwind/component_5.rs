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

	fn render(mut ctx: Ctx<'_>, _props: Example5Props) {
		ctx.styles(vec![Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(vec![Style::Noop("bg-white")]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(vec![Style::Noop("mx-auto"), Style::Noop("max-w-7xl"), Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(96)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))]), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(128))]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(vec![Style::Noop("mx-auto"), Style::Noop("max-w-2xl")]);
					
					props.child("1", Dynamic).run("form", |mut props| {
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(vec![Style::SpaceY(Size::Exact(48))]);
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.styles(vec![Style::Noop("border-b"), Style::Noop("border-gray-900/10"), Style::PaddingBottom(Size::Exact(48))]);
								
								props.child("1", Dynamic).run("h2", |mut props| {
									props.styles(vec![Style::Noop("text-base"), Style::FontSemibold, Style::Noop("leading-7"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.text("Profile"));
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(vec![Style::MarginTop(Size::Exact(4)), Style::Noop("text-sm"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(67))]);
									
									props.child("0", Label).run(|props| props.text("This information will be displayed publicly so be careful what you share."));
								});
								props.child("5", Dynamic).run("div", |mut props| {
									props.styles(vec![Style::MarginTop(Size::Exact(40)), Style::Noop("grid"), Style::Noop("grid-cols-1"), Style::Noop("gap-x-6"), Style::Noop("gap-y-8"), Style::OnScreen(Screen::Small, &[Style::Noop("grid-cols-6")])]);
									
									props.child("1", Dynamic).run("div", |mut props| {
										props.styles(vec![Style::OnScreen(Screen::Small, &[Style::Noop("col-span-4")])]);
										
										props.child("1", Dynamic).run("label", |mut props| {
											props.set_attribute("for", "username");
											props.styles(vec![Style::Block, Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.text("Username"));
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(vec![Style::MarginTop(Size::Exact(8))]);
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(vec![Style::Flex, Style::Noop("rounded-md"), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::NoopGroup("focus-within", &[Style::Noop("ring-2")]), Style::NoopGroup("focus-within", &[Style::Noop("ring-inset")]), Style::NoopGroup("focus-within", &[Style::Noop("ring-indigo-600")]), Style::OnScreen(Screen::Small, &[Style::Noop("max-w-md")])]);
												
												props.child("1", Dynamic).run("span", |mut props| {
													props.styles(vec![Style::Flex, Style::Noop("select-none"), Style::ItemsCenter, Style::PaddingLeft(Size::Exact(12)), Style::TextColor(Color::Fg(56)), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")])]);
													
													props.child("0", Label).run(|props| props.text("workcation.com/"));
												});
												props.child("3", Dynamic).run("input", |mut props| {
													props.set_attribute("type", "text");
													props.set_attribute("name", "username");
													props.set_attribute("id", "username");
													props.set_attribute("autocomplete", "username");
													props.styles(vec![Style::Block, Style::Noop("flex-1"), Style::Noop("border-0"), Style::Noop("bg-transparent"), Style::PaddingY(Size::Exact(6)), Style::PaddingLeft(Size::Exact(4)), Style::TextColor(Color::Fg(100)), Style::NoopGroup("placeholder", &[Style::TextColor(Color::Fg(44))]), Style::OnFocus(&[Style::Noop("ring-0")]), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")]), Style::OnScreen(Screen::Small, &[Style::Noop("leading-6")])]);
													props.set_attribute("placeholder", "janesmith");
												});
											});
										});
									});
									props.child("3", Dynamic).run("div", |mut props| {
										props.styles(vec![Style::Noop("col-span-full")]);
										
										props.child("1", Dynamic).run("label", |mut props| {
											props.set_attribute("for", "about");
											props.styles(vec![Style::Block, Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.text("About"));
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(vec![Style::MarginTop(Size::Exact(8))]);
											
											props.child("1", Dynamic).run("textarea", |mut props| {
												props.set_attribute("id", "about");
												props.set_attribute("name", "about");
												props.set_attribute("rows", "3");
												props.styles(vec![Style::Block, Style::Width(Size::Full), Style::Noop("rounded-md"), Style::Noop("border-0"), Style::PaddingY(Size::Exact(6)), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::NoopGroup("placeholder", &[Style::TextColor(Color::Fg(44))]), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-inset")]), Style::OnFocus(&[Style::Noop("ring-indigo-600")]), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")]), Style::OnScreen(Screen::Small, &[Style::Noop("leading-6")])]);
											});
										});
										props.child("5", Dynamic).run("p", |mut props| {
											props.styles(vec![Style::MarginTop(Size::Exact(12)), Style::Noop("text-sm"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(67))]);
											
											props.child("0", Label).run(|props| props.text("Write a few sentences about yourself."));
										});
									});
									props.child("5", Dynamic).run("div", |mut props| {
										props.styles(vec![Style::Noop("col-span-full")]);
										
										props.child("1", Dynamic).run("label", |mut props| {
											props.set_attribute("for", "photo");
											props.styles(vec![Style::Block, Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.text("Photo"));
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(vec![Style::MarginTop(Size::Exact(8)), Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-3")]);
											
											props.child("1", Icon).run(|props| {
												props.style(vec![Style::Width(Size::Exact(48)), Style::Width(Size::Exact(48)), Style::TextColor(Color::Fg(33))]);
											});
											props.child("3", Dynamic).run("button", |mut props| {
												props.set_attribute("type", "button");
												props.styles(vec![Style::Noop("rounded-md"), Style::Noop("bg-white"), Style::PaddingX(Size::Exact(10)), Style::PaddingY(Size::Exact(6)), Style::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
												
												props.child("0", Label).run(|props| props.text("Change"));
											});
										});
									});
									props.child("7", Dynamic).run("div", |mut props| {
										props.styles(vec![Style::Noop("col-span-full")]);
										
										props.child("1", Dynamic).run("label", |mut props| {
											props.set_attribute("for", "cover-photo");
											props.styles(vec![Style::Block, Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.text("Cover photo"));
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(vec![Style::MarginTop(Size::Exact(8)), Style::Flex, Style::JustifyCenter, Style::Noop("rounded-lg"), Style::Noop("border"), Style::Noop("border-dashed"), Style::Noop("border-gray-900/25"), Style::PaddingX(Size::Exact(24)), Style::PaddingY(Size::Exact(40))]);
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(vec![Style::Noop("text-center")]);
												
												props.child("1", Icon).run(|props| {
													props.style(vec![Style::Noop("mx-auto"), Style::Width(Size::Exact(48)), Style::Width(Size::Exact(48)), Style::TextColor(Color::Fg(33))]);
												});
												props.child("3", Dynamic).run("div", |mut props| {
													props.styles(vec![Style::MarginTop(Size::Exact(16)), Style::Flex, Style::Noop("text-sm"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(67))]);
													
													props.child("1", Dynamic).run("label", |mut props| {
														props.set_attribute("for", "file-upload");
														props.styles(vec![Style::Noop("relative"), Style::Noop("cursor-pointer"), Style::Noop("rounded-md"), Style::Noop("bg-white"), Style::FontSemibold, Style::Noop("text-indigo-600"), Style::NoopGroup("focus-within", &[Style::Noop("outline-none")]), Style::NoopGroup("focus-within", &[Style::Noop("ring-2")]), Style::NoopGroup("focus-within", &[Style::Noop("ring-indigo-600")]), Style::NoopGroup("focus-within", &[Style::Noop("ring-offset-2")]), Style::OnHover(&[Style::Noop("text-indigo-500")])]);
														
														props.child("1", Dynamic).run("span", |_| {});
														props.child("3", Dynamic).run("input", |mut props| {
															props.set_attribute("id", "file-upload");
															props.set_attribute("name", "file-upload");
															props.set_attribute("type", "file");
															props.styles(vec![Style::Noop("sr-only")]);
														});
													});
													props.child("3", Dynamic).run("p", |mut props| {
														props.styles(vec![Style::PaddingLeft(Size::Exact(4))]);
														
														props.child("0", Label).run(|props| props.text("or drag and drop"));
													});
												});
												props.child("5", Dynamic).run("p", |mut props| {
													props.styles(vec![Style::Noop("text-xs"), Style::Noop("leading-5"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.text("PNG, JPG, GIF up to 10MB"));
												});
											});
										});
									});
								});
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(vec![Style::Noop("border-b"), Style::Noop("border-gray-900/10"), Style::PaddingBottom(Size::Exact(48))]);
								
								props.child("1", Dynamic).run("h2", |mut props| {
									props.styles(vec![Style::Noop("text-base"), Style::FontSemibold, Style::Noop("leading-7"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.text("Personal Information"));
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(vec![Style::MarginTop(Size::Exact(4)), Style::Noop("text-sm"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(67))]);
									
									props.child("0", Label).run(|props| props.text("Use a permanent address where you can receive mail."));
								});
								props.child("5", Dynamic).run("div", |mut props| {
									props.styles(vec![Style::MarginTop(Size::Exact(40)), Style::Noop("grid"), Style::Noop("grid-cols-1"), Style::Noop("gap-x-6"), Style::Noop("gap-y-8"), Style::OnScreen(Screen::Small, &[Style::Noop("grid-cols-6")])]);
									
									props.child("1", Dynamic).run("div", |mut props| {
										props.styles(vec![Style::OnScreen(Screen::Small, &[Style::Noop("col-span-3")])]);
										
										props.child("1", Dynamic).run("label", |mut props| {
											props.set_attribute("for", "first-name");
											props.styles(vec![Style::Block, Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.text("First name"));
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(vec![Style::MarginTop(Size::Exact(8))]);
											
											props.child("1", Dynamic).run("input", |mut props| {
												props.set_attribute("type", "text");
												props.set_attribute("name", "first-name");
												props.set_attribute("id", "first-name");
												props.set_attribute("autocomplete", "given-name");
												props.styles(vec![Style::Block, Style::Width(Size::Full), Style::Noop("rounded-md"), Style::Noop("border-0"), Style::PaddingY(Size::Exact(6)), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::NoopGroup("placeholder", &[Style::TextColor(Color::Fg(44))]), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-inset")]), Style::OnFocus(&[Style::Noop("ring-indigo-600")]), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")]), Style::OnScreen(Screen::Small, &[Style::Noop("leading-6")])]);
											});
										});
									});
									props.child("3", Dynamic).run("div", |mut props| {
										props.styles(vec![Style::OnScreen(Screen::Small, &[Style::Noop("col-span-3")])]);
										
										props.child("1", Dynamic).run("label", |mut props| {
											props.set_attribute("for", "last-name");
											props.styles(vec![Style::Block, Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.text("Last name"));
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(vec![Style::MarginTop(Size::Exact(8))]);
											
											props.child("1", Dynamic).run("input", |mut props| {
												props.set_attribute("type", "text");
												props.set_attribute("name", "last-name");
												props.set_attribute("id", "last-name");
												props.set_attribute("autocomplete", "family-name");
												props.styles(vec![Style::Block, Style::Width(Size::Full), Style::Noop("rounded-md"), Style::Noop("border-0"), Style::PaddingY(Size::Exact(6)), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::NoopGroup("placeholder", &[Style::TextColor(Color::Fg(44))]), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-inset")]), Style::OnFocus(&[Style::Noop("ring-indigo-600")]), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")]), Style::OnScreen(Screen::Small, &[Style::Noop("leading-6")])]);
											});
										});
									});
									props.child("5", Dynamic).run("div", |mut props| {
										props.styles(vec![Style::OnScreen(Screen::Small, &[Style::Noop("col-span-4")])]);
										
										props.child("1", Dynamic).run("label", |mut props| {
											props.set_attribute("for", "email");
											props.styles(vec![Style::Block, Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.text("Email address"));
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(vec![Style::MarginTop(Size::Exact(8))]);
											
											props.child("1", Dynamic).run("input", |mut props| {
												props.set_attribute("id", "email");
												props.set_attribute("name", "email");
												props.set_attribute("type", "email");
												props.set_attribute("autocomplete", "email");
												props.styles(vec![Style::Block, Style::Width(Size::Full), Style::Noop("rounded-md"), Style::Noop("border-0"), Style::PaddingY(Size::Exact(6)), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::NoopGroup("placeholder", &[Style::TextColor(Color::Fg(44))]), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-inset")]), Style::OnFocus(&[Style::Noop("ring-indigo-600")]), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")]), Style::OnScreen(Screen::Small, &[Style::Noop("leading-6")])]);
											});
										});
									});
									props.child("7", Dynamic).run("div", |mut props| {
										props.styles(vec![Style::OnScreen(Screen::Small, &[Style::Noop("col-span-3")])]);
										
										props.child("1", Dynamic).run("label", |mut props| {
											props.set_attribute("for", "country");
											props.styles(vec![Style::Block, Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.text("Country"));
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(vec![Style::MarginTop(Size::Exact(8))]);
											
											props.child("1", Dynamic).run("select", |mut props| {
												props.set_attribute("id", "country");
												props.set_attribute("name", "country");
												props.set_attribute("autocomplete", "country-name");
												props.styles(vec![Style::Block, Style::Width(Size::Full), Style::Noop("rounded-md"), Style::Noop("border-0"), Style::PaddingY(Size::Exact(6)), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-inset")]), Style::OnFocus(&[Style::Noop("ring-indigo-600")]), Style::OnScreen(Screen::Small, &[Style::Noop("max-w-xs")]), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")]), Style::OnScreen(Screen::Small, &[Style::Noop("leading-6")])]);
												
												props.child("1", Dynamic).run("option", |_| {});
												props.child("3", Dynamic).run("option", |_| {});
												props.child("5", Dynamic).run("option", |_| {});
											});
										});
									});
									props.child("9", Dynamic).run("div", |mut props| {
										props.styles(vec![Style::Noop("col-span-full")]);
										
										props.child("1", Dynamic).run("label", |mut props| {
											props.set_attribute("for", "street-address");
											props.styles(vec![Style::Block, Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.text("Street address"));
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(vec![Style::MarginTop(Size::Exact(8))]);
											
											props.child("1", Dynamic).run("input", |mut props| {
												props.set_attribute("type", "text");
												props.set_attribute("name", "street-address");
												props.set_attribute("id", "street-address");
												props.set_attribute("autocomplete", "street-address");
												props.styles(vec![Style::Block, Style::Width(Size::Full), Style::Noop("rounded-md"), Style::Noop("border-0"), Style::PaddingY(Size::Exact(6)), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::NoopGroup("placeholder", &[Style::TextColor(Color::Fg(44))]), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-inset")]), Style::OnFocus(&[Style::Noop("ring-indigo-600")]), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")]), Style::OnScreen(Screen::Small, &[Style::Noop("leading-6")])]);
											});
										});
									});
									props.child("11", Dynamic).run("div", |mut props| {
										props.styles(vec![Style::OnScreen(Screen::Small, &[Style::Noop("col-span-2")]), Style::OnScreen(Screen::Small, &[Style::Noop("col-start-1")])]);
										
										props.child("1", Dynamic).run("label", |mut props| {
											props.set_attribute("for", "city");
											props.styles(vec![Style::Block, Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.text("City"));
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(vec![Style::MarginTop(Size::Exact(8))]);
											
											props.child("1", Dynamic).run("input", |mut props| {
												props.set_attribute("type", "text");
												props.set_attribute("name", "city");
												props.set_attribute("id", "city");
												props.set_attribute("autocomplete", "address-level2");
												props.styles(vec![Style::Block, Style::Width(Size::Full), Style::Noop("rounded-md"), Style::Noop("border-0"), Style::PaddingY(Size::Exact(6)), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::NoopGroup("placeholder", &[Style::TextColor(Color::Fg(44))]), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-inset")]), Style::OnFocus(&[Style::Noop("ring-indigo-600")]), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")]), Style::OnScreen(Screen::Small, &[Style::Noop("leading-6")])]);
											});
										});
									});
									props.child("13", Dynamic).run("div", |mut props| {
										props.styles(vec![Style::OnScreen(Screen::Small, &[Style::Noop("col-span-2")])]);
										
										props.child("1", Dynamic).run("label", |mut props| {
											props.set_attribute("for", "region");
											props.styles(vec![Style::Block, Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.text("State / Province"));
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(vec![Style::MarginTop(Size::Exact(8))]);
											
											props.child("1", Dynamic).run("input", |mut props| {
												props.set_attribute("type", "text");
												props.set_attribute("name", "region");
												props.set_attribute("id", "region");
												props.set_attribute("autocomplete", "address-level1");
												props.styles(vec![Style::Block, Style::Width(Size::Full), Style::Noop("rounded-md"), Style::Noop("border-0"), Style::PaddingY(Size::Exact(6)), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::NoopGroup("placeholder", &[Style::TextColor(Color::Fg(44))]), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-inset")]), Style::OnFocus(&[Style::Noop("ring-indigo-600")]), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")]), Style::OnScreen(Screen::Small, &[Style::Noop("leading-6")])]);
											});
										});
									});
									props.child("15", Dynamic).run("div", |mut props| {
										props.styles(vec![Style::OnScreen(Screen::Small, &[Style::Noop("col-span-2")])]);
										
										props.child("1", Dynamic).run("label", |mut props| {
											props.set_attribute("for", "postal-code");
											props.styles(vec![Style::Block, Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.text("ZIP / Postal code"));
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(vec![Style::MarginTop(Size::Exact(8))]);
											
											props.child("1", Dynamic).run("input", |mut props| {
												props.set_attribute("type", "text");
												props.set_attribute("name", "postal-code");
												props.set_attribute("id", "postal-code");
												props.set_attribute("autocomplete", "postal-code");
												props.styles(vec![Style::Block, Style::Width(Size::Full), Style::Noop("rounded-md"), Style::Noop("border-0"), Style::PaddingY(Size::Exact(6)), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::NoopGroup("placeholder", &[Style::TextColor(Color::Fg(44))]), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-inset")]), Style::OnFocus(&[Style::Noop("ring-indigo-600")]), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")]), Style::OnScreen(Screen::Small, &[Style::Noop("leading-6")])]);
											});
										});
									});
								});
							});
							props.child("5", Dynamic).run("div", |mut props| {
								props.styles(vec![Style::Noop("border-b"), Style::Noop("border-gray-900/10"), Style::PaddingBottom(Size::Exact(48))]);
								
								props.child("1", Dynamic).run("h2", |mut props| {
									props.styles(vec![Style::Noop("text-base"), Style::FontSemibold, Style::Noop("leading-7"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.text("Notifications"));
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(vec![Style::MarginTop(Size::Exact(4)), Style::Noop("text-sm"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(67))]);
									
									props.child("0", Label).run(|props| props.text("We'll always let you know about important changes, but you pick what else you want to hear about."));
								});
								props.child("5", Dynamic).run("div", |mut props| {
									props.styles(vec![Style::MarginTop(Size::Exact(40)), Style::SpaceY(Size::Exact(40))]);
									
									props.child("1", Dynamic).run("fieldset", |mut props| {
										props.child("1", Dynamic).run("legend", |mut props| {
											props.styles(vec![Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.text("By Email"));
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(vec![Style::MarginTop(Size::Exact(24)), Style::SpaceY(Size::Exact(24))]);
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(vec![Style::Noop("relative"), Style::Flex, Style::Noop("gap-x-3")]);
												
												props.child("1", Dynamic).run("div", |mut props| {
													props.styles(vec![Style::Flex, Style::Width(Size::Exact(24)), Style::ItemsCenter]);
													
													props.child("1", Dynamic).run("input", |mut props| {
														props.set_attribute("id", "comments");
														props.set_attribute("name", "comments");
														props.set_attribute("type", "checkbox");
														props.styles(vec![Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::RoundedSmall, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-600")])]);
													});
												});
												props.child("3", Dynamic).run("div", |mut props| {
													props.styles(vec![Style::Noop("text-sm"), Style::Noop("leading-6")]);
													
													props.child("1", Dynamic).run("label", |mut props| {
														props.set_attribute("for", "comments");
														props.styles(vec![Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
														
														props.child("0", Label).run(|props| props.text("Comments"));
													});
													props.child("3", Dynamic).run("p", |mut props| {
														props.styles(vec![Style::TextColor(Color::Fg(56))]);
														
														props.child("0", Label).run(|props| props.text("Get notified when someones posts a comment on a posting."));
													});
												});
											});
											props.child("3", Dynamic).run("div", |mut props| {
												props.styles(vec![Style::Noop("relative"), Style::Flex, Style::Noop("gap-x-3")]);
												
												props.child("1", Dynamic).run("div", |mut props| {
													props.styles(vec![Style::Flex, Style::Width(Size::Exact(24)), Style::ItemsCenter]);
													
													props.child("1", Dynamic).run("input", |mut props| {
														props.set_attribute("id", "candidates");
														props.set_attribute("name", "candidates");
														props.set_attribute("type", "checkbox");
														props.styles(vec![Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::RoundedSmall, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-600")])]);
													});
												});
												props.child("3", Dynamic).run("div", |mut props| {
													props.styles(vec![Style::Noop("text-sm"), Style::Noop("leading-6")]);
													
													props.child("1", Dynamic).run("label", |mut props| {
														props.set_attribute("for", "candidates");
														props.styles(vec![Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
														
														props.child("0", Label).run(|props| props.text("Candidates"));
													});
													props.child("3", Dynamic).run("p", |mut props| {
														props.styles(vec![Style::TextColor(Color::Fg(56))]);
														
														props.child("0", Label).run(|props| props.text("Get notified when a candidate applies for a job."));
													});
												});
											});
											props.child("5", Dynamic).run("div", |mut props| {
												props.styles(vec![Style::Noop("relative"), Style::Flex, Style::Noop("gap-x-3")]);
												
												props.child("1", Dynamic).run("div", |mut props| {
													props.styles(vec![Style::Flex, Style::Width(Size::Exact(24)), Style::ItemsCenter]);
													
													props.child("1", Dynamic).run("input", |mut props| {
														props.set_attribute("id", "offers");
														props.set_attribute("name", "offers");
														props.set_attribute("type", "checkbox");
														props.styles(vec![Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::RoundedSmall, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-600")])]);
													});
												});
												props.child("3", Dynamic).run("div", |mut props| {
													props.styles(vec![Style::Noop("text-sm"), Style::Noop("leading-6")]);
													
													props.child("1", Dynamic).run("label", |mut props| {
														props.set_attribute("for", "offers");
														props.styles(vec![Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
														
														props.child("0", Label).run(|props| props.text("Offers"));
													});
													props.child("3", Dynamic).run("p", |mut props| {
														props.styles(vec![Style::TextColor(Color::Fg(56))]);
														
														props.child("0", Label).run(|props| props.text("Get notified when a candidate accepts or rejects an offer."));
													});
												});
											});
										});
									});
									props.child("3", Dynamic).run("fieldset", |mut props| {
										props.child("1", Dynamic).run("legend", |mut props| {
											props.styles(vec![Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.text("Push Notifications"));
										});
										props.child("3", Dynamic).run("p", |mut props| {
											props.styles(vec![Style::MarginTop(Size::Exact(4)), Style::Noop("text-sm"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(67))]);
											
											props.child("0", Label).run(|props| props.text("These are delivered via SMS to your mobile phone."));
										});
										props.child("5", Dynamic).run("div", |mut props| {
											props.styles(vec![Style::MarginTop(Size::Exact(24)), Style::SpaceY(Size::Exact(24))]);
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(vec![Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-3")]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "push-everything");
													props.set_attribute("name", "push-notifications");
													props.set_attribute("type", "radio");
													props.styles(vec![Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-600")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "push-everything");
													props.styles(vec![Style::Block, Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
													
													props.child("0", Label).run(|props| props.text("Everything"));
												});
											});
											props.child("3", Dynamic).run("div", |mut props| {
												props.styles(vec![Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-3")]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "push-email");
													props.set_attribute("name", "push-notifications");
													props.set_attribute("type", "radio");
													props.styles(vec![Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-600")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "push-email");
													props.styles(vec![Style::Block, Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
													
													props.child("0", Label).run(|props| props.text("Same as email"));
												});
											});
											props.child("5", Dynamic).run("div", |mut props| {
												props.styles(vec![Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-3")]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "push-nothing");
													props.set_attribute("name", "push-notifications");
													props.set_attribute("type", "radio");
													props.styles(vec![Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-600")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "push-nothing");
													props.styles(vec![Style::Block, Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
													
													props.child("0", Label).run(|props| props.text("No push notifications"));
												});
											});
										});
									});
								});
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(vec![Style::MarginTop(Size::Exact(24)), Style::Flex, Style::ItemsCenter, Style::Noop("justify-end"), Style::Noop("gap-x-6")]);
							
							props.child("1", Dynamic).run("button", |mut props| {
								props.set_attribute("type", "button");
								props.styles(vec![Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.text("Cancel"));
							});
							props.child("3", Dynamic).run("button", |mut props| {
								props.set_attribute("type", "submit");
								props.styles(vec![Style::Noop("rounded-md"), Style::Noop("bg-indigo-600"), Style::PaddingX(Size::Exact(12)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("text-white"), Style::Noop("shadow-sm"), Style::OnHover(&[Style::Noop("bg-indigo-500")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-2")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-offset-2")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-indigo-600")])]);
								
								props.child("0", Label).run(|props| props.text("Save"));
							});
						});
					});
				});
			});
		});
	}
}
